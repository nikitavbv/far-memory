use {
    std::{net::TcpListener, io::{Read, Write}, fs},
    tracing::{info, error},
    crate::client::RnnReplacementPolicy,
    self::protocol::{ManagerNodeRequest, ManagerNodeResponse, ReplacementPolicyType, ReplacementPolicyParams},
};

pub use self::{client::Client as ManagerClient, protocol::SpanAccessEvent};

mod client;
mod protocol;

const REQ_SIZE_LIMIT: u64 = 10 * 1024 * 1024 * 1024;
const SPAN_ACCESS_STATS_FILE: &str = "./data/span_access_stats.json";

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

    span_access_stats: Vec<SpanAccessEvent>,
}

impl Server {
    pub fn new(token: String) -> Self {
        Self {
            auth: false,
            token,

            span_access_stats: Vec::new(),
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
            ManagerNodeRequest::GetReplacementPolicyParams(policy_type) => {
                if !self.auth {
                    return ManagerNodeResponse::Forbidden;
                }

                let span_access_history = fs::read(SPAN_ACCESS_STATS_FILE).ok().map(|v| serde_json::from_slice(&v).unwrap());

                match policy_type {
                    ReplacementPolicyType::Replay => ManagerNodeResponse::ReplacementPolicyParams(ReplacementPolicyParams {
                        span_access_history,
                        rnn_weights: None,
                    }),
                    ReplacementPolicyType::RNN => ManagerNodeResponse::ReplacementPolicyParams(ReplacementPolicyParams {
                        span_access_history: None,
                        rnn_weights: span_access_history.map(RnnReplacementPolicy::train_rnn_model),
                    }),
                }
            },
            ManagerNodeRequest::SpanAccessStats(mut stats) => {
                if !self.auth {
                    return ManagerNodeResponse::Forbidden;
                }

                self.span_access_stats.append(&mut stats);
                ManagerNodeResponse::Ok
            },
            ManagerNodeRequest::FinishSession => {
                if !self.auth {
                    return ManagerNodeResponse::Forbidden;
                }

                fs::write(SPAN_ACCESS_STATS_FILE, serde_json::to_vec(&self.span_access_stats).unwrap()).unwrap();
                ManagerNodeResponse::Ok
            }
        }
    }
}
