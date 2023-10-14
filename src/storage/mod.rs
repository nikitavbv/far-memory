use {
    std::{net::{TcpListener, TcpStream, Shutdown}, io::Write, collections::HashMap},
    tracing::info,
    self::protocol::{StorageRequest, StorageResponse},
};

mod protocol;

pub fn run_storage_server() {
    info!("running storage server");
    run_server("some-token".to_owned(), None, None);
}

fn run_server(token: String, connections_limit: Option<usize>, requests_limit: Option<usize>) {
    let listener = TcpListener::bind("0.0.0.0:14000").unwrap();

    let mut connections = 0;
    for stream in listener.incoming() {
        println!("waiting for next connection");
        let mut stream = stream.unwrap();
        connections += 1;

        let mut server = Server::new(token.clone());

        info!("handling incoming connection");
        let mut requests = 0;
        loop {
            requests += 1;
            if let Some(limit) = requests_limit {
                if requests > limit {
                    break;
                }
            }

            let req: StorageRequest = bincode::deserialize_from(&stream).unwrap();     
            let res = server.handle(req);
            stream.write(&bincode::serialize(&res).unwrap()).unwrap();
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
        Self {
            stream: TcpStream::connect(addr).unwrap(),
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
            StorageResponse::SwapIn { span_id, data } => data,
            other => panic!("unexpected swap in response: {:?}", other),
        }
    }

    fn request(&mut self, request: StorageRequest) -> StorageResponse {
        self.write_request(request);
        self.read_response()
    }

    fn write_request(&mut self, request: StorageRequest) {
        self.stream.write(&bincode::serialize(&request).unwrap()).unwrap();
    }

    fn read_response(&mut self) -> StorageResponse {
        bincode::deserialize_from(&self.stream).unwrap()
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
        let server_thread = thread::spawn(|| run_server("some-token".to_owned(), Some(1), Some(3)));
        let mut client = Client::new("localhost:14000");
        
        client.auth("some-token");
        client.swap_out(42, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        let res = client.swap_in(42);

        assert_eq!(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], res);

        server_thread.join().unwrap();
    }
}