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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StorageResponse {
    Ok,
    Forbidden,
    SwapIn {
        span_id: u64,
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
        data_len: u64,
    },
    Batch(Vec<StorageResponse>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapOutRequest {
    pub span_id: u64,
    pub prepend: bool,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
