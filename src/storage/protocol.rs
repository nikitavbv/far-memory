use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StorageRequest {
    Auth {
        token: String,
    },
    SetRunId {
        run_id: String,
    },
    SwapOut(SwapOutRequest),
    SwapIn {
        span_id: u64,
    },
    Batch(Vec<StorageRequest>),
    IgnoreResponse(Box<StorageRequest>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StorageResponse {
    Ok,
    Nop,
    Forbidden,
    SwapIn {
        span_id: u64,
        data: Vec<u8>,
    },
    Batch(Vec<StorageResponse>),
    Ack,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapOutRequest {
    pub span_id: u64,
    pub prepend: bool,
    pub data: Vec<u8>,
}

impl StorageRequest {
    pub fn is_ignore_response(&self) -> bool {
        match self {
            Self::IgnoreResponse(_) => true,
            _ => false,
        }
    }
}
