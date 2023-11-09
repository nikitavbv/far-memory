use {
    std::{net::TcpListener, io::{Read, Write}},
    tracing::{info, error},
    self::protocol::{ManagerNodeRequest, ManagerNodeResponse},
};

pub use self::client::Client as ManagerClient;

mod client;
mod protocol;

const REQ_SIZE_LIMIT: u64 = 10 * 1024 * 1024 * 1024;

pub fn run_manager_node(token: String) {
    let port = 14000;
    info!("running manager node on port {}", port);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut server = Server::new(token.clone());

        info!("handling incoming connection");
        loop {
            let req_len = {
                let mut req_len: [u8; 8] = [0u8; 8];
                if let Err(err) = stream.read(&mut req_len) {
                    error!("unexpected error when reading request header: {:?}", err);
                    break;
                }
                u64::from_be_bytes(req_len)
            };

            let req = {
                if req_len > REQ_SIZE_LIMIT {
                    error!("request is too large");
                    break;
                }

                let mut req = vec![0u8; req_len as usize];
                if let Err(err) = stream.read_exact(&mut req) {
                    error!("unexpected error when reading request body: {:?}", err);
                    break;
                }

                req
            };

            let req = match bincode::deserialize(&req) {
                Ok(v) => v,
                Err(err) => {
                    error!("unexpected error when reading request: {:?}", err);
                    break;
                }
            };

            let res = server.handle(req);

            let res = bincode::serialize(&res).unwrap();
            stream.write(&(res.len() as u64).to_be_bytes()).unwrap();
            stream.write(&res).unwrap();
        }
    }
}

struct Server {
    auth: bool,
    token: String,
}

impl Server {
    pub fn new(token: String) -> Self {
        Self {
            auth: false,
            token,
        }
    }

    fn handle(&mut self, req: ManagerNodeRequest) -> ManagerNodeResponse {
        match req {
            ManagerNodeRequest::Auth { token } => {
                self.auth = self.token == token;
                if self.auth {
                    ManagerNodeResponse::Ok
                } else {
                    ManagerNodeResponse::Forbidden
                }
            },
            ManagerNodeRequest::SpanAccessStats(stats) => {
                info!("received span access stats: {} entries", stats.len());
                ManagerNodeResponse::Ok
            }
        }
    }
}
