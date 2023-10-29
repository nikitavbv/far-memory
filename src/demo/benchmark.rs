use {
    std::time::Instant,
    tokio::{net::TcpListener, io::{AsyncWriteExt, AsyncReadExt}},
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

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let listener = TcpListener::bind("0.0.0.0:14000").await.unwrap();

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            socket.set_nodelay(true).unwrap();

            let mut rng = rand::thread_rng();
            let data: Vec<u8> = (0..180355072).map(|_| rng.gen()).collect();

            let (mut reader, mut writer) = socket.split();

            let started_at = Instant::now();
            let data_len = data.len();
            let write_response_task = async move {
                writer.write(&(data.len() as u64).to_be_bytes()).await.unwrap();
                writer.write(&data).await.unwrap();
            };

            let read_request_task = async move {
                let req_len = {
                    let mut req_len: [u8; 8] = [0u8; 8];
                    reader.read_exact(&mut req_len).await.unwrap();
                    u64::from_be_bytes(req_len)
                };

                {
                    let mut req = vec![0u8; req_len as usize];
                    reader.read_exact(&mut req).await.unwrap();
                    req
                }
            };

            let (_, res) = tokio::join!(write_response_task, read_request_task);
            info!("processed request in: {:?}, received {}, sent {}", Instant::now() - started_at, res.len(), data_len);
        }
    });
}

fn run_client(endpoint: String) {
    unimplemented!()
}
