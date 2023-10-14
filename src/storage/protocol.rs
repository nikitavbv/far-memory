use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StorageRequest {
    Auth {
        token: String,
    },
    SwapOut {
        span_id: u64,
        data: Vec<u8>,
    },
    SwapIn {
        span_id: u64,
    },
}

#[derive(Serialize, Deserialize)]
pub enum StorageResponse {
    Ok,
    SwapIn {
        span_id: u64,
        data: Vec<u8>,
    },
}