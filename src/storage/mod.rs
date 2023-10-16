use {
    std::{
        net::{TcpListener, TcpStream, Shutdown}, 
        io::{Write, Read}, 
        collections::HashMap, 
        time::Duration, 
        thread,
    },
    tracing::{info, error, span, Level},
    self::protocol::{StorageRequest, StorageResponse},
};

mod protocol;

pub fn run_storage_server(token: String) {
    info!("running storage server");
    run_server("0.0.0.0".to_owned(), token, None, None);
}

fn run_server(host: String, token: String, connections_limit: Option<usize>, requests_limit: Option<usize>) {
    let listener = TcpListener::bind(format!("{}:14000", host)).unwrap();

    let mut connections = 0;
    for stream in listener.incoming() {
        println!("waiting for next connection");
        let mut stream = stream.unwrap();
        connections += 1;

        let mut server = Server::new(token.clone());

        info!("handling incoming connection");
        let mut requests = 0;
        loop {
            let _req_loop_span = span!(Level::DEBUG, "request loop iteration").entered();

            requests += 1;
            if let Some(limit) = requests_limit {
                if requests > limit {
                    break;
                }
            }

            let req_len = {
                let _req_len_span = span!(Level::DEBUG, "read request header").entered();

                let mut req_len: [u8; 8] = [0u8; 8];
                if let Err(err) = stream.read(&mut req_len) {
                    error!("unexpected error when reading request header: {:?}", err);
                    break;
                }
                u64::from_be_bytes(req_len)
            };

            let req = {
                let _req_body_span = span!(Level::DEBUG, "read request body").entered();

                let mut req = vec![0u8; req_len as usize];
                if let Err(err) = stream.read_exact(&mut req) {
                    error!("unexpected error when reading request body: {:?}", err);
                    break;
                };

                req
            };

            let req = {
                let _req_deserialize_body = span!(Level::DEBUG, "deserialize request body").entered();

                match bincode::deserialize(&req) {
                    Ok(v) => v,
                    Err(err) => {
                        error!("unexpected error when reading request: {:?}", err);
                        break;
                    }
                }
            };

            let res = span!(Level::DEBUG, "handle request").in_scope(|| server.handle(req));

            let res = span!(Level::DEBUG, "serialize response").in_scope(|| bincode::serialize(&res).unwrap());

            span!(Level::DEBUG, "write response").in_scope(|| {
                stream.write(&(res.len() as u64).to_be_bytes()).unwrap();
                stream.write(&res).unwrap();
            });
        }

        if let Some(limit) = connections_limit {
            if connections >= limit {
                break;
            }
        }
    }
}

pub struct Server {
    auth: bool,
    token: String,

    spans: HashMap<u64, Vec<u8>>,
}

impl Server {
    pub fn new(token: String) -> Self {
        Self {
            auth: false,
            token,

            spans: HashMap::new(),
        }
    }

    pub fn handle(&mut self, req: StorageRequest) -> StorageResponse {
        match req {
            StorageRequest::Auth { token } => {
                self.auth = self.token == token;
                if self.auth {
                    StorageResponse::Ok
                } else {
                    StorageResponse::Forbidden
                }
            },
            StorageRequest::SwapOut { span_id, data } => {
                if !self.auth {
                    return StorageResponse::Forbidden;
                }

                self.spans.insert(span_id, data);

                StorageResponse::Ok
            },
            StorageRequest::SwapIn { span_id } => {
                if !self.auth {
                    return StorageResponse::Forbidden;
                }

                let data = self.spans.remove(&span_id).unwrap();

                StorageResponse::SwapIn { span_id, data }
            },
        }
    }
}

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        let mut stream = TcpStream::connect(addr);
        while !stream.is_ok() {
            eprintln!("connection failed: {:?}", stream.err().unwrap());
            thread::sleep(Duration::from_secs(1));
            stream = TcpStream::connect(addr);
        }

        Self {
            stream: stream.unwrap(),
        }
    }

    pub fn auth(&mut self, token: &str) {
        match self.request(StorageRequest::Auth {
            token: token.to_owned(),
        }) {
            StorageResponse::Ok => (),
            other => panic!("unexpected auth response: {:?}", other),
        }
    }

    pub fn swap_out(&mut self, span_id: u64, data: Vec<u8>) {
        match self.request(StorageRequest::SwapOut { span_id, data }) {
            StorageResponse::Ok => (),
            other => panic!("unexpected swap out response: {:?}", other),
        }
    }

    pub fn swap_in(&mut self, span_id: u64) -> Vec<u8> {
        match self.request(StorageRequest::SwapIn { span_id }) {
            StorageResponse::SwapIn { span_id: _, data } => data,
            other => panic!("unexpected swap in response: {:?}", other),
        }
    }

    fn request(&mut self, request: StorageRequest) -> StorageResponse {
        span!(Level::DEBUG, "writing request").in_scope(|| {
            self.write_request(request);
        });
        span!(Level::DEBUG, "reading response").in_scope(|| {
            self.read_response()
        })
    }

    fn write_request(&mut self, request: StorageRequest) {
        let serialized = span!(Level::DEBUG, "serialize").in_scope(|| bincode::serialize(&request).unwrap());

        span!(Level::DEBUG, "write header").in_scope(|| self.stream.write(&(serialized.len() as u64).to_be_bytes()).unwrap());
        span!(Level::DEBUG, "write data").in_scope(|| self.stream.write(&serialized).unwrap());
    }

    fn read_response(&mut self) -> StorageResponse {
        let res_len = span!(Level::DEBUG, "reading response header").in_scope(|| {
            let mut res_len: [u8; 8] = [0u8; 8];
            self.stream.read_exact(&mut res_len).unwrap();
            u64::from_be_bytes(res_len)
        });

        let res = span!(Level::DEBUG, "reading response body").in_scope(|| {
            let mut res = vec![0u8; res_len as usize];
            self.stream.read_exact(&mut res).unwrap();
            res
        });

        span!(Level::DEBUG, "deserialize").in_scope(|| bincode::deserialize(&res).unwrap())
    }

    pub fn close(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use {
        std::thread,
        super::*,
    };

    #[test]
    fn simple() {
        let server_thread = thread::spawn(|| run_server("localhost".to_owned(), "some-token".to_owned(), Some(1), Some(3)));
        let mut client = Client::new("localhost:14000");
        
        client.auth("some-token");
        client.swap_out(42, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        let res = client.swap_in(42);

        assert_eq!(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], res);

        server_thread.join().unwrap();
    }
}