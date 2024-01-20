use {
    std::collections::HashMap,
    tracing::{info, error, span, Level},
    tokio::{net::{TcpSocket, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}},
    prometheus::{Registry, register_int_counter_vec_with_registry, IntCounterVec, IntGaugeVec, register_int_gauge_vec_with_registry},
    thiserror::Error,
    async_recursion::async_recursion,
    self::protocol::{StorageRequest, StorageRequestBody, StorageResponse},
};

pub use self::{
    protocol::{SwapOutRequest, SpanData},
    client::{Client, BatchSwapOutOperation, LocalSpanData},
};

const REQ_SIZE_LIMIT: u64 = 10 * 1024 * 1024 * 1024;
pub const BUFFER_SIZE: u32 = 128 * 1024;

mod client;
mod protocol;

#[derive(Error, Debug)]
enum StorageServerError {
    #[error("failed to create server socket")]
    FailedToCreateServerSocket,
    #[error("failed to read span data")]
    FailedToReadSpanData,
}

pub fn run_storage_server(metrics: Registry, token: String, port: Option<u16>) {
    run_server(Some(metrics), "0.0.0.0".to_owned(), port, token, None, None).unwrap();
}

fn run_server(metrics: Option<Registry>, host: String, port: Option<u16>, token: String, connections_limit: Option<usize>, requests_limit: Option<usize>) -> Result<(), StorageServerError> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let port = port.unwrap_or(14001);
        let hostname = hostname::get().unwrap().to_str().unwrap().to_owned();
        let addr = format!("{}:{}", host, port);

        let socket = TcpSocket::new_v4().unwrap();
        if let Err(err) = socket.bind(addr.parse().unwrap()) {
            error!("failed to create server socket: {:?}", err);
            return Err(StorageServerError::FailedToCreateServerSocket);
        }
        socket.set_recv_buffer_size(BUFFER_SIZE).unwrap();
        socket.set_send_buffer_size(BUFFER_SIZE).unwrap();
        socket.set_reuseaddr(true).unwrap();

        let listener = socket.listen(1024).unwrap();

        info!("running storage server on {}", addr);

        let metrics = metrics.map(|v| ServerMetrics::new(v));

        let mut connections = 0;
        while let Ok ((mut stream, _add)) = listener.accept().await {
            connections += 1;

            stream.set_nodelay(true).unwrap();
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
                    if let Err(err) = stream.read(&mut req_len).await {
                        error!("unexpected error when reading request header: {:?}", err);
                        break;
                    }
                    u64::from_be_bytes(req_len)
                };

                let req = {
                    let _req_body_span = span!(Level::DEBUG, "read request body").entered();

                    if req_len > REQ_SIZE_LIMIT {
                        error!("request is too large!");
                        break;
                    }

                    let mut req = vec![0u8; req_len as usize];
                    if let Err(err) = stream.read_exact(&mut req).await {
                        error!("unexpected error when reading request body: {:?}", err);
                        break;
                    }

                    req
                };

                let mut req: StorageRequest = {
                    let _req_deserialize_body = span!(Level::DEBUG, "deserialize request body").entered();

                    match bincode::deserialize(&req) {
                        Ok(v) => v,
                        Err(err) => {
                            error!("unexpected error when reading request: {:?}", err);
                            break;
                        }
                    }
                };

                let request_id = req.request_id;
                req.body = match inline_span_data_into_request(req.body, &mut stream).await {
                    Ok(v) => v,
                    Err(err) => {
                        error!("failed to inline span data into request: {:?}", err);
                        break;
                    }
                };

                let res = span!(Level::DEBUG, "handle request", request_id).in_scope(|| server.handle(req.body));
                let (res, span_data) = match res {
                    StorageResponse::SwapIn { span_id, data } => {
                        let span_data = match data {
                            SpanData::Inline(data) => vec![data],
                            SpanData::Concat { data } => data,
                            _ => panic!("didn't expect data to be external at this point"),
                        };

                        (StorageResponse::SwapIn { span_id, data: SpanData::External { len: span_data.iter().map(|v| v.len() as u64).sum() } }, Some(span_data))
                    },
                    StorageResponse::Batch(responses) => {
                        let mut span_data = None;

                        let mut new_responses = Vec::new();
                        for response in responses {
                            let new_response = match response {
                                StorageResponse::SwapIn { span_id, data } => {
                                    span_data = Some(match data {
                                        SpanData::Inline(data) => vec![data],
                                        SpanData::Concat { data } => data,
                                        _ => panic!("didn't expect data to be external at this point"),
                                    });

                                    StorageResponse::SwapIn { span_id, data: SpanData::External { len: span_data.as_ref().map(|data| data.iter().map(|v| v.len() as u64).sum()).unwrap() } }
                                },
                                other => other
                            };
                            new_responses.push(new_response);
                        }

                        (StorageResponse::Batch(new_responses), span_data)
                    },
                    other => (other, None),
                };

                let res = span!(Level::DEBUG, "serialize response", request_id).in_scope(|| bincode::serialize(&res).unwrap());

                let scope_write_response = span!(Level::DEBUG, "write response", request_id);
                let _scope_write_response_guard = scope_write_response.enter();

                stream.write_all(&(res.len() as u64).to_be_bytes()).await.unwrap();
                stream.write_all(&res).await.unwrap();

                if let Some(span_data) = span_data {
                    for chunk in span_data {
                        stream.write_all(&chunk).await.unwrap();
                    }
                }
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

        Ok(())
    })?;

    Ok(())
}

pub struct Server {
    auth: bool,
    token: String,

    spans: HashMap<u64, Vec<Vec<u8>>>,

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

    pub fn handle(&mut self, req: StorageRequestBody) -> StorageResponse {
        match req {
            StorageRequestBody::Auth { token } => {
                self.auth = self.token == token;
                if self.auth {
                    StorageResponse::Ok
                } else {
                    StorageResponse::Forbidden
                }
            },
            StorageRequestBody::SetRunId { run_id } => {
                self.run_id = run_id;
                StorageResponse::Ok
            }
            StorageRequestBody::SwapOut(swap_out_req) => span!(Level::DEBUG, "handling swap out request").in_scope(|| {
                if !self.auth {
                    return StorageResponse::Forbidden;
                }

                let data = match swap_out_req.data {
                    SpanData::Inline(data) => data,
                    _ => panic!("expected span data to be inline"),
                };
                let bytes_swapped_out = data.len();

                span!(Level::DEBUG, "inserting into spans").in_scope(|| if swap_out_req.prepend {
                    self.spans.get_mut(&swap_out_req.span_id).unwrap().insert(0, data);
                } else {
                    self.spans.insert(swap_out_req.span_id, vec![data]);
                });

                if let Some(metrics) = self.metrics.as_ref() {
                    metrics.total_spans.with_label_values(&[&self.addr, &self.run_id]).set(self.spans.len() as i64);
                    metrics.total_bytes.with_label_values(&[&self.addr, &self.run_id]).set(self.total_span_bytes() as i64);

                    metrics.swap_out_operations.with_label_values(&[&self.addr, &self.run_id]).inc();
                    metrics.swap_out_bytes.with_label_values(&[&self.addr, &self.run_id]).inc_by(bytes_swapped_out as u64);
                }

                StorageResponse::Ok
            }),
            StorageRequestBody::SwapIn { span_id } => span!(Level::DEBUG, "handling swap in request").in_scope(|| {
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

                StorageResponse::SwapIn { span_id, data: SpanData::Concat { data } }
            }),
            StorageRequestBody::Batch(reqs) => span!(Level::DEBUG, "handling batch request").in_scope(|| {
                let res = reqs.into_iter().map(|req| self.handle(req)).collect();
                StorageResponse::Batch(res)
            }),
        }
    }

    fn total_span_bytes(&self) -> usize {
        self.spans.iter().map(|v| v.1.len()).sum()
    }
}

#[async_recursion]
async fn inline_span_data_into_request(request: StorageRequestBody, stream: &mut TcpStream) -> Result<StorageRequestBody, StorageServerError> {
    Ok(match request {
        StorageRequestBody::SwapOut(swap_out_request) => {
            let data = match swap_out_request.data {
                SpanData::Inline(data) => SpanData::Inline(data),
                SpanData::Concat { data } => SpanData::Inline(data.concat()),
                SpanData::External { len } => SpanData::Inline({
                    let mut data = vec![0; len as usize];
                    if let Err(_err) = stream.read_exact(&mut data).await {
                        return Err(StorageServerError::FailedToReadSpanData)
                    }
                    data
                })
            };

            StorageRequestBody::SwapOut(SwapOutRequest {
                data,
                ..swap_out_request
            })
        },
        StorageRequestBody::Batch(reqs) => {
            let mut result = Vec::new();
            for req in reqs {
                result.push(inline_span_data_into_request(req, stream).await?);
            }
            StorageRequestBody::Batch(result)
        },
        other => other,
    })
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
            "127.0.0.1".to_owned(),
            Some(14000),
            "some-token".to_owned(),
            Some(1),
            Some(3)
        ).unwrap());
        let mut client = Client::new("localhost:14000");

        client.auth("some-token");
        client.swap_out(42, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], false);
        let res = client.swap_in(42);

        assert_eq!(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], res);

        server_thread.join().unwrap();
    }

    #[test]
    fn prepend() {
        let server_thread = thread::spawn(|| run_server(
            None,
            "127.0.0.1".to_owned(),
            Some(14001),
            "some-token".to_owned(),
            Some(1),
            Some(4)
        ).unwrap());
        let mut client = Client::new("localhost:14001");

        client.auth("some-token");

        client.swap_out(42, vec![10, 9, 8], false);
        client.swap_out(42, vec![7, 6, 5], true);

        let res = client.swap_in(42);

        assert_eq!(vec![7, 6, 5, 10, 9, 8], res);

        server_thread.join().unwrap();
    }
}
