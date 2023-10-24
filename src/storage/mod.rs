use {
    std::{
        net::{TcpListener, TcpStream, Shutdown},
        io::{Write, Read},
        collections::HashMap,
        time::Duration,
        thread,
    },
    tracing::{info, error, span, Level},
    prometheus::{Registry, register_int_counter_vec_with_registry, IntCounterVec, IntGaugeVec, register_int_gauge_vec_with_registry},
    self::protocol::{StorageRequest, StorageResponse},
};

pub use self::protocol::SwapOutRequest;

const REQ_SIZE_LIMIT: u64 = 10 * 1024 * 1024 * 1024;

mod protocol;

pub fn run_storage_server(metrics: Registry, token: String, port: Option<u16>) {
    run_server(Some(metrics), "0.0.0.0".to_owned(), port, token, None, None);
}

fn run_server(metrics: Option<Registry>, host: String, port: Option<u16>, token: String, connections_limit: Option<usize>, requests_limit: Option<usize>) {
    let port = port.unwrap_or(14000);
    let hostname = hostname::get().unwrap().to_str().unwrap().to_owned();
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).unwrap();

    info!("running storage server on {}", addr);

    let metrics = metrics.map(|v| ServerMetrics::new(v));

    let mut connections = 0;
    for stream in listener.incoming() {
        println!("waiting for next connection");
        let mut stream = stream.unwrap();
        connections += 1;

        let mut server = Server::new(metrics.clone(), format!("{}:{}", hostname, port), token.clone());

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

                if req_len > REQ_SIZE_LIMIT {
                    panic!("request is too large!");
                }

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

        if let Some(metrics) = metrics.as_ref() {
            metrics.reset();
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

    metrics: Option<ServerMetrics>,
    addr: String,
    run_id: String,
}

#[derive(Clone)]
pub struct ServerMetrics {
    total_spans: IntGaugeVec,
    total_bytes: IntGaugeVec,

    swap_out_operations: IntCounterVec,
    swap_out_bytes: IntCounterVec,

    swap_in_operations: IntCounterVec,
    swap_in_bytes: IntCounterVec,
}

impl ServerMetrics {
    pub fn new(registry: Registry) -> Self {
        Self {
            total_spans: register_int_gauge_vec_with_registry!(
                "storage_spans",
                "total spans in memory of this node",
                &["server_addr", "run_id"],
                registry
            ).unwrap(),
            total_bytes: register_int_gauge_vec_with_registry!(
                "storage_bytes",
                "total bytes in memory of this node",
                &["server_addr", "run_id"],
                registry
            ).unwrap(),

            swap_out_operations: register_int_counter_vec_with_registry!(
                "storage_swap_out_ops",
                "total swap out requests",
                &["server_addr", "run_id"],
                registry
            ).unwrap(),
            swap_out_bytes: register_int_counter_vec_with_registry!(
                "storage_swap_out_bytes",
                "total bytes swapped out",
                &["server_addr", "run_id"],
                registry
            ).unwrap(),

            swap_in_operations: register_int_counter_vec_with_registry!(
                "storage_swap_in_ops",
                "total swap in operations",
                &["server_addr", "run_id"],
                registry
            ).unwrap(),
            swap_in_bytes: register_int_counter_vec_with_registry!(
              "storage_swap_in_bytes",
              "total swap in bytes",
              &["server_addr", "run_id"],
              registry
            ).unwrap(),
        }
    }

    pub fn reset(&self) {
        self.total_spans.reset();
        self.total_bytes.reset();

        self.swap_out_operations.reset();
        self.swap_out_bytes.reset();

        self.swap_in_operations.reset();
        self.swap_in_bytes.reset();
    }
}

impl Server {
    pub fn new(metrics: Option<ServerMetrics>, addr: String, token: String) -> Self {
        Self {
            auth: false,
            token,

            spans: HashMap::new(),

            metrics,
            addr,
            run_id: "unknown".to_owned(),
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
            StorageRequest::SetRunId { run_id } => {
                self.run_id = run_id;
                StorageResponse::Ok
            }
            StorageRequest::SwapOut(swap_out_req) => {
                if !self.auth {
                    return StorageResponse::Forbidden;
                }

                let bytes_swapped_out = swap_out_req.data.len();

                let existing = self.spans.insert(swap_out_req.span_id, swap_out_req.data);
                if swap_out_req.prepend {
                    self.spans.get_mut(&swap_out_req.span_id).unwrap().append(&mut existing.unwrap());
                }

                if let Some(metrics) = self.metrics.as_ref() {
                    metrics.total_spans.with_label_values(&[&self.addr, &self.run_id]).set(self.spans.len() as i64);
                    metrics.total_bytes.with_label_values(&[&self.addr, &self.run_id]).set(self.total_span_bytes() as i64);

                    metrics.swap_out_operations.with_label_values(&[&self.addr, &self.run_id]).inc();
                    metrics.swap_out_bytes.with_label_values(&[&self.addr, &self.run_id]).inc_by(bytes_swapped_out as u64);
                }

                StorageResponse::Ok
            },
            StorageRequest::SwapIn { span_id } => {
                if !self.auth {
                    return StorageResponse::Forbidden;
                }

                let data = self.spans.remove(&span_id).unwrap();

                if let Some(metrics) = self.metrics.as_ref() {
                    metrics.total_spans.with_label_values(&[&self.addr, &self.run_id]).set(self.spans.len() as i64);
                    metrics.total_bytes.with_label_values(&[&self.addr, &self.run_id]).set(self.total_span_bytes() as i64);

                    metrics.swap_in_operations.with_label_values(&[&self.addr, &self.run_id]).inc();
                    metrics.swap_in_bytes.with_label_values(&[&self.addr, &self.run_id]).inc_by(data.len() as u64);
                }

                StorageResponse::SwapIn { span_id, data }
            },
            StorageRequest::Batch(reqs) => {
                let res = reqs.into_iter().map(|req| self.handle(req)).collect();
                StorageResponse::Batch(res)
            },
        }
    }

    fn total_span_bytes(&self) -> usize {
        self.spans.iter().map(|v| v.1.len()).sum()
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

    pub fn set_run_id(&mut self, run_id: String) {
        match self.request(StorageRequest::SetRunId {
            run_id,
        }) {
            StorageResponse::Ok => (),
            other => panic!("unexpected set run id response: {:?}", other),
        }
    }

    pub fn swap_out(&mut self, span_id: u64, data: Vec<u8>, prepend: bool) {
        match self.request(StorageRequest::SwapOut(SwapOutRequest { span_id, prepend, data })) {
            StorageResponse::Ok => (),
            other => panic!("unexpected swap out response: {:?}", other),
        }
    }
    
    pub fn batch_swap_out(&mut self, req: Vec<SwapOutRequest>) {
        let reqs = req.into_iter().map(|v| StorageRequest::SwapOut(SwapOutRequest { span_id: v.span_id, prepend: v.prepend, data: v.data })).collect();
        let req = StorageRequest::Batch(reqs);
        match self.request(req) {
            StorageResponse::Batch(responses) => for res in responses {
                match res {
                    StorageResponse::Ok => (),
                    other => panic!("unexpected one of batch swap out responses: {:?}", other),
                }
            },
            other => panic!("unexpected batch swap out response: {:?}", other),
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
        let server_thread = thread::spawn(|| run_server(
            None,
            "localhost".to_owned(),
            None,
            "some-token".to_owned(),
            Some(1),
            Some(3)
        ));
        let mut client = Client::new("localhost:14000");

        client.auth("some-token");
        client.swap_out(42, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], false);
        let res = client.swap_in(42);

        assert_eq!(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], res);

        server_thread.join().unwrap();
    }
}
