use {
    std::{thread, time::Duration, io::{Write, Read}, sync::atomic::{AtomicU64, Ordering}},
    tracing::{span, Level},
    tokio::{net::{TcpStream, TcpSocket}, io::{AsyncReadExt, AsyncWriteExt}},
    super::protocol::{StorageRequest, StorageRequestBody, StorageResponse, SpanData, SwapOutRequest},
};

pub struct Client {
    stream: TcpStream,
    request_id: AtomicU64,
}

impl Client {
    pub async fn new(addr: &str) -> Self {
        let socket = TcpSocket::new_v4().unwrap();
        let mut stream = socket.connect(addr.parse().unwrap()).await;
        while !stream.is_ok() {
            eprintln!("connection failed: {:?}", stream.err().unwrap());
            thread::sleep(Duration::from_secs(1));
            stream = TcpStream::connect(addr).await;
        }
        let stream = stream.unwrap();
        stream.set_nodelay(true).unwrap();

        Self {
            stream,
            request_id: AtomicU64::new(0),
        }
    }

    pub async fn auth(&mut self, token: &str) {
        match self.request(StorageRequestBody::Auth {
            token: token.to_owned(),
        }).await {
            StorageResponse::Ok => (),
            other => panic!("unexpected auth response: {:?}", other),
        }
    }

    pub async fn set_run_id(&mut self, run_id: String) {
        match self.request(StorageRequestBody::SetRunId {
            run_id,
        }).await {
            StorageResponse::Ok => (),
            other => panic!("unexpected set run id response: {:?}", other),
        }
    }

    pub async fn swap_out(&mut self, span_id: u64, data: Vec<u8>, prepend: bool) {
        match self.request(StorageRequestBody::SwapOut(SwapOutRequest { span_id, prepend, data: SpanData::Inline(data) })).await {
            StorageResponse::Ok => (),
            other => panic!("unexpected swap out response: {:?}", other),
        }
    }

    pub async fn batch(&mut self, swap_out: Vec<BatchSwapOutOperation>, swap_in: Option<u64>) -> Option<Vec<u8>> {
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

        match self.request_with_external_span_data(req, local_span_data).await {
            StorageResponse::Batch(responses) => for res in responses {
                match res {
                    StorageResponse::Ok => (),
                    StorageResponse::SwapIn { span_id: _, data } => swap_in_result = Some(match data {
                       SpanData::Inline(data) => data,
                       SpanData::Concat { data } => data.concat(),
                       SpanData::External { len } => {
                           let reading_span = span!(Level::DEBUG, "reading span body", len);
                           let _reading_span_guard = reading_span.enter();
                           let mut data = vec![0u8; len as usize];
                           self.stream.read_exact(&mut data).await.unwrap();
                           data
                        },
                    }),
                    other => panic!("unexpected one of batch swap out responses: {:?}", other),
                }
            },
            other => panic!("unexpected batch swap out response: {:?}", other),
        };

        swap_in_result
    }

    pub async fn swap_in(&mut self, span_id: u64) -> Vec<u8> {
        let data = match self.request(StorageRequestBody::SwapIn { span_id }).await {
            StorageResponse::SwapIn { span_id: _, data } => data,
            other => panic!("unexpected swap in response: {:?}", other),
        };

        match data {
            SpanData::Inline(data) => data,
            SpanData::Concat { data } => data.concat(),
            SpanData::External { len } => {
                let reading_span = span!(Level::DEBUG, "reading span body", len);
                let _reading_span_scope = reading_span.enter();
                let mut data = vec![0u8; len as usize];
                self.stream.read_exact(&mut data).await.unwrap();
                data
            },
        }
    }

    async fn request(&mut self, request: StorageRequestBody) -> StorageResponse {
        let request_id = self.next_request_id();

        let writing_request_span = span!(Level::DEBUG, "writing request", request_id);
        let writing_request_guard = writing_request_span.enter();
        self.write_request(StorageRequest { body: request, request_id }).await;
        drop(writing_request_guard);

        let reading_response_span = span!(Level::DEBUG, "reading response");
        let _reading_response_guard = reading_response_span.enter();
        self.read_response().await
    }

    async fn request_with_external_span_data(&mut self, body: StorageRequestBody, span_data: Vec<LocalSpanData>) -> StorageResponse {
        let request_id = self.next_request_id();

        let writing_request_span = span!(Level::DEBUG, "writing request", request_id);
        let writing_request_guard = writing_request_span.enter();
        self.write_request_with_external_span_data(StorageRequest { body, request_id }, span_data).await;
        drop(writing_request_guard);

        let reading_response_span = span!(Level::DEBUG, "reading response");
        let _reading_response_guard = reading_response_span.enter();
        self.read_response().await
    }

    async fn write_request(&mut self, mut request: StorageRequest) {
        let mut span_data = Vec::new();
        request.body = extract_span_data_from_request(request.body, &mut span_data);
        self.write_request_with_external_span_data(request, span_data).await
    }

    async fn write_request_with_external_span_data(&mut self, request: StorageRequest, span_data: Vec<LocalSpanData>) {
        let serialized = span!(Level::DEBUG, "serialize").in_scope(|| bincode::serialize(&request).unwrap());

        let write_header_span = span!(Level::DEBUG, "write header");
        let write_header_span_guard = write_header_span.enter();
        self.stream.write(&(serialized.len() as u64).to_be_bytes()).await.unwrap();
        drop(write_header_span_guard);

        let write_data_span = span!(Level::DEBUG, "write data");
        let write_data_span_guard = write_data_span.enter();
        self.stream.write(&serialized).await.unwrap();
        drop(write_data_span_guard);

        let write_span_data_span = span!(Level::DEBUG, "write span data");
        let write_span_data_guard = write_span_data_span.enter();
        for v in span_data.iter() {
            let writing_to_stream_span = span!(Level::DEBUG, "writing to stream");
            let _writing_to_stream_guard = writing_to_stream_span.enter();
            self.stream.write(v.as_slice()).await.unwrap();
        }
        drop(write_span_data_guard);

        span!(Level::DEBUG, "dropping local span data").in_scope(|| drop(span_data));
    }

    async fn read_response(&mut self) -> StorageResponse {
        let reading_response_header_span = span!(Level::DEBUG, "reading response header");
        let reading_response_header_guard = reading_response_header_span.enter();
        let mut res_len: [u8; 8] = [0u8; 8];
        self.stream.read_exact(&mut res_len).await.unwrap();
        let res_len = u64::from_be_bytes(res_len);
        drop(reading_response_header_guard);

        let reading_response_body_span = span!(Level::DEBUG, "reading response body");
        let reading_response_body_guard = reading_response_body_span.enter();
        let mut res = vec![0u8; res_len as usize];
        self.stream.read_exact(&mut res).await.unwrap();
        drop(reading_response_body_guard);

        span!(Level::DEBUG, "deserialize").in_scope(|| bincode::deserialize(&res).unwrap())
    }

    pub async fn close(&mut self) {
        self.stream.shutdown().await.unwrap();
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
