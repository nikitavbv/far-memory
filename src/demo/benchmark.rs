use {
    std::{net::{TcpListener, TcpStream}, io::{Read, Write, ErrorKind}, time::Instant},
    rand::Rng,
    crossbeam::utils::Backoff,
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
        stream.set_nonblocking(true).unwrap();

        loop {
            let req_len = span!(Level::DEBUG, "read request header").in_scope(|| {
                let mut req_len: [u8; 8] = [0u8; 8];
                let backoff = Backoff::new();

                loop {
                    match stream.read_exact(&mut req_len) {
                        Ok(_) => {
                            break;
                        },
                        Err(err) => {
                            if err.kind() == ErrorKind::WouldBlock {
                                backoff.spin();
                            } else {
                                panic!("error while reading: {:?}", err);
                            }
                        }
                    };
                }
                u64::from_be_bytes(req_len)
            });

            let started_at = Instant::now();

            span!(Level::DEBUG, "write response").in_scope(|| {
                stream.write(&(data.len() as u64).to_be_bytes()).unwrap();
                stream.write(&data).unwrap();
            });

            let req = span!(Level::DEBUG, "read request body").in_scope(|| {
                let mut req = vec![0u8; req_len as usize];
                let backoff = Backoff::new();

                loop {
                    match stream.read_exact(&mut req) {
                        Ok(_) => {
                            break;
                        },
                        Err(err) => {
                            if err.kind() == ErrorKind::WouldBlock {
                                backoff.spin();
                            } else {
                                panic!("error while reading: {:?}", err);
                            }
                        }
                    };
                }

                req
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
    stream.set_nonblocking(true).unwrap();

    let started_at = Instant::now();

    span!(Level::DEBUG, "write header").in_scope(|| stream.write(&(data.len() as u64).to_be_bytes()).unwrap());
    span!(Level::DEBUG, "write data").in_scope(|| stream.write(&data).unwrap());

    info!("writing data took {:?}", Instant::now() - started_at);

    let res_len = span!(Level::DEBUG, "reading response header").in_scope(|| {
        let mut res_len: [u8; 8] = [0u8; 8];
        let backoff = Backoff::new();

        loop {
            match stream.read_exact(&mut res_len) {
                Ok(_) => {
                    break;
                },
                Err(err) => {
                    if err.kind() == ErrorKind::WouldBlock {
                        backoff.spin();
                    } else {
                        panic!("error while reading: {:?}", err);
                    }
                }
            };
        }

        u64::from_be_bytes(res_len)
    });

    let res = span!(Level::DEBUG, "reading response body").in_scope(|| {
        let mut res = vec![0u8; res_len as usize];

        let backoff = Backoff::new();

        loop {
            match stream.read_exact(&mut res) {
                Ok(_) => {
                    break;
                },
                Err(err) => {
                    if err.kind() == ErrorKind::WouldBlock {
                        backoff.spin();
                    } else {
                        panic!("error while reading: {:?}", err);
                    }
                }
            };
        }

        res
    });

    info!("finished in: {:?}, received {}, sent {}", Instant::now() - started_at, res.len(), data.len());
}
