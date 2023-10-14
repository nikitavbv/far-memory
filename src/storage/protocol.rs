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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StorageResponse {
    Ok,
    Forbidden,
    SwapIn {
        span_id: u64,
        data: Vec<u8>,
    },
}