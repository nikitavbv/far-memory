use {
    tracing::info,
    crate::client::{NetworkNodeBackend, FarMemoryBackend, SpanId},
};

pub fn run_benchmark(token: &str, endpoint: &str) {
    info!("running benchmark");

    let mut client = NetworkNodeBackend::new(endpoint, token);

    let data = vec![0u8; 100 * 1024 * 1024];

    info!("let's swap out");
    client.swap_out(SpanId::from_id(42), &data);

    info!("done swapping out");
}