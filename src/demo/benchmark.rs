use {
    std::{time::Instant, net::{TcpListener, TcpStream}, io::{Read, Write}},
    rand::RngCore,
    tracing::{info, span, Level},
};

const DATA_TO_TRANSFER: usize = 200 * 1024 * 1024; // 200MB is typical payload

pub fn run_benchmark(token: &str, endpoint: Option<String>) {
    info!("running benchmark");

    if let Some(endpoint) = endpoint {
        run_client(endpoint);
    } else {
        run_server();
    }
}

fn run_server() {
    info!("running benchmark server");

    let listener = TcpListener::bind("0.0.0.0:14000").unwrap();
    let mut buffer = vec![0u8; DATA_TO_TRANSFER];

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let started_at = Instant::now();
        stream.read_exact(&mut buffer).unwrap();
        info!("server finished reading in {:?}", Instant::now() - started_at);

        let started_at = Instant::now();
        stream.write_all(&buffer).unwrap();
        info!("server finished writing in {:?}", Instant::now() - started_at);
    }
}

fn run_client(endpoint: String) {
    let mut data = vec![0u8; DATA_TO_TRANSFER];
    rand::thread_rng().fill_bytes(&mut data);

    let mut stream = TcpStream::connect(&endpoint).unwrap();
    stream.set_nodelay(true).unwrap();

    let started_at = Instant::now();
    stream.write_all(&data).unwrap();
    info!("client finished writing in {}mbps", DATA_TO_TRANSFER as f64 / (Instant::now() - started_at).as_secs_f64() * 8.0 / 1000.0 / 1000.0);

    let started_at = Instant::now();
    stream.read_exact(&mut data).unwrap();
    info!("client finished reading in {:?}", Instant::now() - started_at);
}
