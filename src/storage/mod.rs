use {
    std::{
        net::{TcpListener, TcpStream, Shutdown}, 
        io::{Write, Read}, 
        collections::HashMap, 
        sync::atomic::{AtomicU64, Ordering}, 
        time::{Instant, Duration}, 
        thread,
    },
    tracing::{info, error},
    self::protocol::{StorageRequest, StorageResponse},
};

mod protocol;

static SERVER_TIMER_DESERIALIZE: AtomicU64 = AtomicU64::new(0);
static SERVER_TIMER_HANDLE: AtomicU64 = AtomicU64::new(0);

static CLIENT_TIMER_SWAP_OUT_SERIALIZE: AtomicU64 = AtomicU64::new(0);
static CLIENT_TIMER_SWAP_OUT_SEND: AtomicU64 = AtomicU64::new(0);

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
            requests += 1;
            if let Some(limit) = requests_limit {
                if requests > limit {
                    break;
                }
            }

            let mut req_len = [0u8; 8];
            stream.read(&mut req_len).unwrap();

            let req_len = u64::from_be_bytes(req_len);
            let mut req = vec![0u8; req_len as usize];
            stream.read_exact(&mut req).unwrap();

            let started_at = Instant::now();
            let req: StorageRequest = match bincode::deserialize(&req) {
                Ok(v) => v,
                Err(err) => {
                    error!("Unexpected error when reading request: {:?}", err);
                    break;
                }
            };     
            SERVER_TIMER_DESERIALIZE.fetch_add((Instant::now() - started_at).as_millis() as u64, Ordering::Relaxed);

            let started_at = Instant::now();
            let res = server.handle(req);
            SERVER_TIMER_HANDLE.fetch_add((Instant::now() - started_at).as_millis() as u64, Ordering::Relaxed);

            stream.write(&bincode::serialize(&res).unwrap()).unwrap();
        }

        print_server_performance_report();

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
        self.write_request(request);
        self.read_response()
    }

    fn write_request(&mut self, request: StorageRequest) {
        let started_at = Instant::now();
        let serialized = bincode::serialize(&request).unwrap();
        CLIENT_TIMER_SWAP_OUT_SERIALIZE.fetch_add((Instant::now() - started_at).as_millis() as u64, Ordering::Relaxed);

        let started_at = Instant::now();
        self.stream.write(&(serialized.len() as u64).to_be_bytes()).unwrap();
        self.stream.write(&serialized).unwrap();
        CLIENT_TIMER_SWAP_OUT_SEND.fetch_add((Instant::now() - started_at).as_millis() as u64, Ordering::Relaxed);
    }

    fn read_response(&mut self) -> StorageResponse {
        bincode::deserialize_from(&self.stream).unwrap()
    }

    pub fn close(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}

pub fn print_client_performance_report() {
    println!("swap out serialize: {:?}, swap out send: {:?}", CLIENT_TIMER_SWAP_OUT_SERIALIZE, CLIENT_TIMER_SWAP_OUT_SEND);
}

pub fn print_server_performance_report() {
    println!("deserialize: {:?}, handle: {:?}", SERVER_TIMER_DESERIALIZE, SERVER_TIMER_HANDLE);
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