use {
    std::{net::TcpStream, thread, io::{Read, Write}, time::Duration, sync::{Mutex, Arc, atomic::{AtomicBool, AtomicU64, Ordering}}},
    crate::client::SpanId,
    super::protocol::{ManagerNodeRequest, ManagerNodeResponse},
};

pub struct Client {
    stream: Arc<Mutex<TcpStream>>,
    is_running: Arc<AtomicBool>,

    time_step_counter: AtomicU64,
    span_access_stats: Arc<Mutex<Vec<SpanAccessStatsEntry>>>,

}

struct SpanAccessStatsEntry {
    span_id: SpanId,
    time_step: u64,
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
        let stream = Arc::new(Mutex::new(stream.unwrap()));

        let is_running = Arc::new(AtomicBool::new(true));
        let span_access_stats = Arc::new(Mutex::new(Vec::new()));

        thread::Builder::new().name("manager-client".to_owned()).spawn(manager_client_thread(is_running.clone(), stream.clone())).unwrap();
        Self {
            stream,
            is_running,
            time_step_counter: AtomicU64::new(0),
            span_access_stats,
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

    pub fn on_stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    pub fn on_span_access(&self, span_id: &SpanId) {
        // TODO: should time step be provided externally? How prediction would work?
        let time_step = self.time_step_counter.fetch_add(1, Ordering::Relaxed);

        unimplemented!()
    }

    fn request(&mut self, request: ManagerNodeRequest) -> ManagerNodeResponse {
        let mut stream = self.stream.lock().unwrap();
        write_request(&mut stream, request);
        read_response(&mut stream)
    }
}

fn manager_client_thread(is_running: Arc<AtomicBool>, stream: Arc<Mutex<TcpStream>>) -> impl FnOnce() -> () {
    move || {
        while is_running.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(5));
        }
    }
}

fn write_request(stream: &mut TcpStream, request: ManagerNodeRequest) {
    let serialized = bincode::serialize(&request).unwrap();

    stream.write(&(serialized.len() as u64).to_be_bytes()).unwrap();
    stream.write(&serialized).unwrap();
}

fn read_response(stream: &mut TcpStream) -> ManagerNodeResponse {
    let res_len = {
        let mut res_len: [u8; 8] = [0u8; 8];
        stream.read_exact(&mut res_len).unwrap();
        u64::from_be_bytes(res_len)
    };

    let res = {
        let mut res = vec![0u8; res_len as usize];
        stream.read_exact(&mut res).unwrap();
        res
    };

    bincode::deserialize(&res).unwrap()
}