use {
    tracing::info,
    crate::client::{NetworkNodeBackend, FarMemoryBackend, SpanId},
};

pub fn run_benchmark(token: &str, endpoint: &str) {
    info!("running benchmark");

    let client = NetworkNodeBackend::new(endpoint, token);

    let mut data = vec![0u8; 100 * 1024 * 1024];
    for i in 0..data.len() {
        data[i] = rand::random();
    }

    info!("let's swap out");
    client.swap_out(SpanId::from_id(42), &data);
    info!("done swapping out");

    info!("let's swap in");
    let data = client.swap_in(&SpanId::from_id(42));
    info!("done swapping in: {:?}", data.len());
}