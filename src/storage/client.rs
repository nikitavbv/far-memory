use {
    std::{net::{TcpStream, Shutdown}, thread, time::Duration, io::{Write, Read}},
    tracing::{span, Level},
    super::protocol::{StorageRequest, StorageResponse, SpanData, SwapOutRequest},
};

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
        let mut stream = stream.unwrap();
        stream.set_nodelay(true).unwrap();

        Self {
            stream,
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
        match self.request(StorageRequest::SwapOut(SwapOutRequest { span_id, prepend, data: SpanData::Inline(data) })) {
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
        let data = match self.request(StorageRequest::SwapIn { span_id }) {
            StorageResponse::SwapIn { span_id: _, data } => data,
            other => panic!("unexpected swap in response: {:?}", other),
        };

        match data {
            SpanData::Inline(data) => data,
            SpanData::External { len } => span!(Level::DEBUG, "reading span body").in_scope(|| {
                let mut data = vec![0u8; len as usize];
                self.stream.read_exact(&mut data).unwrap();
                data
            }),
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
