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
    pub data: SpanData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SpanData {
    Inline(#[serde(with = "serde_bytes")] Vec<u8>),
    External {
        len: u64,
    },
}

pub enum InlineSpanData {
    Owned(Vec<u8>),
    ReadFrom {
        ptr: *mut u8,
        size: usize,
    }
}
