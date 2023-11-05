use {
    std::{net::TcpStream, thread, io::{Read, Write}, time::Duration},
    crate::client::SpanId,
    super::protocol::{ManagerNodeRequest, ManagerNodeResponse},
};

pub struct Client {
    stream: TcpStream,
}

// this client tends to have both high-level logic and communication layer. It probably needs to be split into two separate components. The client itself and manager logic.
impl Client {
    pub fn new(addr: &str) -> Self {
        let mut stream = TcpStream::connect(addr);
        while !stream.is_ok() {
            eprintln!("connection to manager node failed: {:?}", stream.err().unwrap());
            thread::sleep(Duration::from_secs(1));
            stream = TcpStream::connect(addr);
        }
        let stream = stream.unwrap();

        Self {
            stream,
        }
    }

    pub fn auth(&mut self, token: &str) {
        match self.request(ManagerNodeRequest::Auth {
            token: token.to_owned(),
        }) {
            ManagerNodeResponse::Ok => (),
            other => panic!("unexpected auth response from manager node: {:?}", other),
        }
    }

    pub fn on_span_access(&self, span_id: &SpanId) {
        unimplemented!()
    }

    fn request(&mut self, request: ManagerNodeRequest) -> ManagerNodeResponse {
        self.write_request(request);
        self.read_response()
    }

    fn write_request(&mut self, request: ManagerNodeRequest) {
        let serialized = bincode::serialize(&request).unwrap();
        self.stream.write(&(serialized.len() as u64).to_be_bytes()).unwrap();
        self.stream.write(&serialized).unwrap();
    }

    fn read_response(&mut self) -> ManagerNodeResponse {
        let res_len = {
            let mut res_len: [u8; 8] = [0u8; 8];
            self.stream.read_exact(&mut res_len).unwrap();
            u64::from_be_bytes(res_len)
        };

        let res = {
            let mut res = vec![0u8; res_len as usize];
            self.stream.read_exact(&mut res).unwrap();
            res
        };

        bincode::deserialize(&res).unwrap()
    }
}
