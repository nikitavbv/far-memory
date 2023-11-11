use {
    std::{net::TcpStream, thread, io::{Read, Write}, time::Duration, sync::{Mutex, Arc, atomic::{AtomicBool, Ordering}}},
    crate::client::SpanId,
    super::protocol::{ManagerNodeRequest, ManagerNodeResponse, SpanAccessEvent, ReplacementPolicyType, ReplacementPolicyParams},
};

#[derive(Clone)]
pub struct Client {
    stream: Arc<Mutex<TcpStream>>,
    is_running: Arc<AtomicBool>,

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

        thread::Builder::new().name("manager-client".to_owned()).spawn(manager_client_thread(
            is_running.clone(),
            stream.clone(),
            span_access_stats.clone(),
        )).unwrap();
        Self {
            stream,
            is_running,
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
        push_span_access_stats(&self.stream, &self.span_access_stats);
        match self.request(ManagerNodeRequest::FinishSession) {
            ManagerNodeResponse::Ok => (),
            other => panic!("unexpected finish session response: {:?}", other),
        }
    }

    pub fn on_span_access(&self, span_id: &SpanId, time_step: u64) {
        self.span_access_stats.lock().unwrap().push(SpanAccessStatsEntry { span_id: span_id.clone(), time_step });
    }

    pub fn get_replacement_policy_params(&self) -> ReplacementPolicyParams {
        match self.request(ManagerNodeRequest::GetReplacementPolicyParams(ReplacementPolicyType::Replay)) {
            ManagerNodeResponse::ReplacementPolicyParams(params) => params,
            other => panic!("unexpected get replacement policy params response: {:?}", other),
        }
    }

    fn request(&self, req: ManagerNodeRequest) -> ManagerNodeResponse {
        request(&mut self.stream.lock().unwrap(), req)
    }
}

fn manager_client_thread(
    is_running: Arc<AtomicBool>,
    stream: Arc<Mutex<TcpStream>>,
    span_access_stats: Arc<Mutex<Vec<SpanAccessStatsEntry>>>
) -> impl FnOnce() -> () {
    move || {
        while is_running.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(5));
            push_span_access_stats(&stream, &span_access_stats);
        }
    }
}

fn push_span_access_stats(stream: &Arc<Mutex<TcpStream>>, span_access_stats: &Arc<Mutex<Vec<SpanAccessStatsEntry>>>) {
    let span_access_stats = {
        let mut stats = Vec::new();
        let mut span_access_stats = span_access_stats.lock().unwrap();
        std::mem::swap(&mut stats, &mut span_access_stats);
        stats
    };

    if span_access_stats.is_empty() {
        return;
    }

    let span_access_stats: Vec<_> = span_access_stats.into_iter()
        .map(|stat| SpanAccessEvent {
            span_id: stat.span_id.id(),
            time_step: stat.time_step,
        })
        .collect();

    let req = ManagerNodeRequest::SpanAccessStats(span_access_stats);
    match request(&mut stream.lock().unwrap(), req) {
        ManagerNodeResponse::Ok => (),
        other => panic!("unexpected response from manager node when sending span access stats: {:?}", other),
    };
}

fn request(stream: &mut TcpStream, request: ManagerNodeRequest) -> ManagerNodeResponse {
    write_request(stream, request);
    read_response(stream)
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
