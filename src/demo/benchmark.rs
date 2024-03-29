use {
    std::time::Instant,
    tokio::{net::{TcpStream, TcpListener, TcpSocket}, io::{AsyncReadExt, AsyncWriteExt}},
    rand::RngCore,
    tracing::info,
};

const DATA_TO_TRANSFER: usize = 200 * 1024 * 1024; // 200MB is typical payload
const BUFFER_SIZE: u32 = 512 * 1024;

pub fn run_benchmark(token: &str, endpoint: Option<String>) {
    info!("running benchmark");

    let rt = tokio::runtime::Runtime::new().unwrap();

    if let Some(endpoint) = endpoint {
        rt.block_on(async {
            run_client(endpoint).await;
        })
    } else {
        rt.block_on(async {
            run_server().await;
        });
    }
}

async fn run_server() {
    info!("running benchmark server");

    let socket = TcpSocket::new_v4().unwrap();
    socket.bind("0.0.0.0:14000".parse().unwrap()).unwrap();
    socket.set_reuseaddr(true).unwrap();
    socket.set_recv_buffer_size(BUFFER_SIZE).unwrap();
    socket.set_send_buffer_size(BUFFER_SIZE).unwrap();

    let listener = socket.listen(1).unwrap();
    let mut buffer = vec![0u8; DATA_TO_TRANSFER];

    while let Ok((mut stream, _addr)) = listener.accept().await {
        let started_at = Instant::now();
        stream.read_exact(&mut buffer).await.unwrap();
        info!("server finished reading in {:?}", Instant::now() - started_at);

        let started_at = Instant::now();
        stream.write_all(&buffer).await.unwrap();
        info!("server finished writing in {:?}", Instant::now() - started_at);
    }
}

async fn run_client(endpoint: String) {
    let mut data = vec![0u8; DATA_TO_TRANSFER];
    rand::thread_rng().fill_bytes(&mut data);

    let socket = TcpSocket::new_v4().unwrap();
    socket.set_recv_buffer_size(BUFFER_SIZE).unwrap();
    socket.set_send_buffer_size(BUFFER_SIZE).unwrap();
    info!("send buffer size: {:?}", socket.send_buffer_size());

    let mut stream = socket.connect(endpoint.parse().unwrap()).await.unwrap();
    stream.set_nodelay(true).unwrap();

    let started_at = Instant::now();
    stream.write_all(&data).await.unwrap();
    info!("client finished writing in {}mbps", DATA_TO_TRANSFER as f64 / (Instant::now() - started_at).as_secs_f64() * 8.0 / 1000.0 / 1000.0);

    let started_at = Instant::now();
    stream.read_exact(&mut data).await.unwrap();
    info!("client finished reading in {:?}", Instant::now() - started_at);
}
