use {
    std::{net::{TcpListener, TcpStream}, io::{Read, Write}, time::Instant},
    rand::Rng,
    tracing::{info, span, Level},
};

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

    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..180355072).map(|_| rng.gen()).collect();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.set_nodelay(true).unwrap();

        loop {
            let req_len = span!(Level::DEBUG, "read request header").in_scope(|| {
                let mut req_len: [u8; 8] = [0u8; 8];
                if let Err(err) = stream.read(&mut req_len) {
                    panic!("unexpected error when reading request header: {:?}", err);
                }
                u64::from_be_bytes(req_len)
            });

            let started_at = Instant::now();

            let req = span!(Level::DEBUG, "read request body").in_scope(|| {
                let mut req = vec![0u8; req_len as usize];
                if let Err(err) = stream.read_exact(&mut req) {
                    panic!("unexpected error when reading request body: {:?}", err);
                };

                req
            });

            span!(Level::DEBUG, "write response").in_scope(|| {
                stream.write(&(data.len() as u64).to_be_bytes()).unwrap();
                stream.write(&data).unwrap();
            });

            info!("processed request in: {:?}, received {}, sent {}", Instant::now() - started_at, req.len(), data.len());
        }
    }
}

fn run_client(endpoint: String) {
    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..180355072).map(|_| rng.gen()).collect();

    let mut stream = TcpStream::connect(endpoint).unwrap();
    stream.set_nodelay(true).unwrap();

    let started_at = Instant::now();

    span!(Level::DEBUG, "write header").in_scope(|| stream.write(&(data.len() as u64).to_be_bytes()).unwrap());
    span!(Level::DEBUG, "write data").in_scope(|| stream.write(&data).unwrap());

    let res_len = span!(Level::DEBUG, "reading response header").in_scope(|| {
        let mut res_len: [u8; 8] = [0u8; 8];
        stream.read_exact(&mut res_len).unwrap();
        u64::from_be_bytes(res_len)
    });

    let res = span!(Level::DEBUG, "reading response body").in_scope(|| {
        let mut res = vec![0u8; res_len as usize];
        stream.read_exact(&mut res).unwrap();
        res
    });

    info!("finished in: {:?}, received {}, sent {}", Instant::now() - started_at, res.len(), data.len());
}
