use {
    tracing::info,
    super::byte_buffer::FarMemoryByteBuffer,
};

pub async fn run_test_mode(endpoint: String, token: String, far_memory_block_size: u64) {
    info!("running test mode");

    let mut buffer = FarMemoryByteBuffer::new(endpoint, token, far_memory_block_size).await;
    buffer.write(0, &[1, 2, 3, 4, 5]).await;

    let mut bytes = vec![0; 10];
    buffer.read(0, &mut bytes).await;

    info!("bytes are: {:?}", bytes);
}