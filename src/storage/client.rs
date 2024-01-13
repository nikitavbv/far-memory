use {
    std::{net::{TcpStream, Shutdown}, thread, time::Duration, io::{Write, Read}, sync::atomic::{AtomicU64, Ordering}},
    tracing::{span, Level},
    super::protocol::{StorageRequest, StorageRequestBody, StorageResponse, SpanData, SwapOutRequest},
};

pub struct Client {
    stream: TcpStream,
    request_id: AtomicU64,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        let mut stream = TcpStream::connect(addr);
        while !stream.is_ok() {
            eprintln!("connection failed: {:?}", stream.err().unwrap());
            thread::sleep(Duration::from_secs(1));
            stream = TcpStream::connect(addr);
        }
        let stream = stream.unwrap();
        stream.set_nodelay(true).unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();

        Self {
            stream,
            request_id: AtomicU64::new(0),
        }
    }

    pub fn auth(&mut self, token: &str) {
        match self.request(StorageRequestBody::Auth {
            token: token.to_owned(),
        }) {
            StorageResponse::Ok => (),
            other => panic!("unexpected auth response: {:?}", other),
        }
    }

    pub fn set_run_id(&mut self, run_id: String) {
        match self.request(StorageRequestBody::SetRunId {
            run_id,
        }) {
            StorageResponse::Ok => (),
            other => panic!("unexpected set run id response: {:?}", other),
        }
    }

    pub fn swap_out(&mut self, span_id: u64, data: Vec<u8>, prepend: bool) {
        match self.request(StorageRequestBody::SwapOut(SwapOutRequest { span_id, prepend, data: SpanData::Inline(data) })) {
            StorageResponse::Ok => (),
            other => panic!("unexpected swap out response: {:?}", other),
        }
    }

    pub fn batch(&mut self, swap_out: Vec<BatchSwapOutOperation>, swap_in: Option<u64>) -> Option<Vec<u8>> {
        let mut reqs: Vec<_> = swap_out.iter().map(|v| StorageRequestBody::SwapOut(SwapOutRequest {
            span_id: v.span_id,
            prepend: v.prepend,
            data: SpanData::External { len: v.data.len() },
        })).collect();
        let local_span_data: Vec<_> = swap_out.into_iter().map(|v| v.data).collect();
        if let Some(span_id) = swap_in {
            reqs.push(StorageRequestBody::SwapIn { span_id });
        }

        let req = StorageRequestBody::Batch(reqs);

        let mut swap_in_result = None;

        match self.request_with_external_span_data(req, local_span_data) {
            StorageResponse::Batch(responses) => for res in responses {
                match res {
                    StorageResponse::Ok => (),
                    StorageResponse::SwapIn { span_id: _, data } => swap_in_result = Some(match data {
                       SpanData::Inline(data) => data,
                       SpanData::Concat { data } => data.concat(),
                       SpanData::External { len } => span!(Level::DEBUG, "reading span body", len).in_scope(|| {
                           let mut data = vec![0u8; len as usize];
                           self.stream.read_exact(&mut data).unwrap();
                           data
                       }),
                    }),
                    other => panic!("unexpected one of batch swap out responses: {:?}", other),
                }
            },
            other => panic!("unexpected batch swap out response: {:?}", other),
        };

        swap_in_result
    }

    pub fn swap_in(&mut self, span_id: u64) -> Vec<u8> {
        let data = match self.request(StorageRequestBody::SwapIn { span_id }) {
            StorageResponse::SwapIn { span_id: _, data } => data,
            other => panic!("unexpected swap in response: {:?}", other),
        };

        match data {
            SpanData::Inline(data) => data,
            SpanData::Concat { data } => data.concat(),
            SpanData::External { len } => span!(Level::DEBUG, "reading span body", len).in_scope(|| {
                let mut data = vec![0u8; len as usize];
                self.stream.read_exact(&mut data).unwrap();
                data
            }),
        }
    }

    fn request(&mut self, request: StorageRequestBody) -> StorageResponse {
        let request_id = self.next_request_id();

        span!(Level::DEBUG, "writing request", request_id).in_scope(|| {
            self.write_request(StorageRequest { body: request, request_id });
        });
        span!(Level::DEBUG, "reading response").in_scope(|| {
            self.read_response()
        })
    }

    fn request_with_external_span_data(&mut self, body: StorageRequestBody, span_data: Vec<LocalSpanData>) -> StorageResponse {
        let request_id = self.next_request_id();

        span!(Level::DEBUG, "writing request", request_id).in_scope(|| {
            self.write_request_with_external_span_data(StorageRequest { body, request_id }, span_data);
        });
        span!(Level::DEBUG, "reading response").in_scope(|| {
            self.read_response()
        })
    }

    fn write_request(&mut self, mut request: StorageRequest) {
        let mut span_data = Vec::new();
        request.body = extract_span_data_from_request(request.body, &mut span_data);
        self.write_request_with_external_span_data(request, span_data)
    }

    fn write_request_with_external_span_data(&mut self, request: StorageRequest, span_data: Vec<LocalSpanData>) {
        let serialized = span!(Level::DEBUG, "serialize").in_scope(|| bincode::serialize(&request).unwrap());

        span!(Level::DEBUG, "write header").in_scope(|| self.stream.write(&(serialized.len() as u64).to_be_bytes()).unwrap());
        span!(Level::DEBUG, "write data").in_scope(|| self.stream.write(&serialized).unwrap());
        span!(Level::DEBUG, "writing span data").in_scope(|| span_data.into_iter().for_each(|v| { self.stream.write(v.as_slice()).unwrap(); }));
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

    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::Relaxed)
    }
}

pub enum LocalSpanData {
    Owned(Vec<u8>),
    ReadFrom {
        ptr: *mut u8,
        size: usize,
    }
}

impl LocalSpanData {
    fn as_slice(&self) -> &[u8] {
        match self {
            Self::Owned(data) => &data,
            Self::ReadFrom { ptr, size } => unsafe {
                std::slice::from_raw_parts(*ptr, *size)
            }
        }
    }

    fn len(&self) -> u64 {
        match self {
            Self::Owned(data) => data.len() as u64,
            Self::ReadFrom { ptr: _, size } => *size as u64,
        }
    }
}

pub struct BatchSwapOutOperation {
    pub span_id: u64,
    pub data: LocalSpanData,
    pub prepend: bool,
}

fn extract_span_data_from_request(request: StorageRequestBody, span_data: &mut Vec<LocalSpanData>) -> StorageRequestBody {
    match request {
        StorageRequestBody::SwapOut(swap_out_request) => {
            let len = match swap_out_request.data {
                SpanData::Inline(data) => {
                    let len = data.len();
                    span_data.push(LocalSpanData::Owned(data));
                    len as u64
                },
                _ => panic!("expected span data to be inline"),
            };

            StorageRequestBody::SwapOut(SwapOutRequest {
                data: SpanData::External { len },
                ..swap_out_request
            })
        },
        StorageRequestBody::Batch(reqs) => StorageRequestBody::Batch(reqs.into_iter().map(|v| extract_span_data_from_request(v, span_data)).collect()),
        other => other,
    }
}
