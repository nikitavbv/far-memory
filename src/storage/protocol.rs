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
        data: SpanData,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SpanData {
    Inline(#[serde(with = "serde_bytes")] Vec<u8>),
    External {
        len: u64,
    }
}
