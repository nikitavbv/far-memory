use {
    tracing::info,
    super::{
        controller_client::ControllerClient,
        byte_buffer::FarMemoryByteBuffer,
    },
};

pub async fn run_test_mode(endpoint: String, token: String, far_memory_block_size: u64) {
    info!("running test mode");

    let client = ControllerClient::new(endpoint, token).await;

    let mut buffer = FarMemoryByteBuffer::new(client, far_memory_block_size).await;
    buffer.write(1, &[1, 2, 3, 4, 5]).await;

    let mut bytes = vec![0; 10];
    buffer.read(0, &mut bytes).await;

    info!("bytes are: {:?}", bytes);
}