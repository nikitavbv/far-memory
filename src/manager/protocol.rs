use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ManagerNodeRequest {
    Auth {
        token: String,
    },
    SpanAccessStats(Vec<SpanAccessEvent>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ManagerNodeResponse {
    Ok,
    Forbidden,
}

#[derive(Serialize, Deserialize)]
pub struct SpanAccessEvent {
    time_step: u64,
    span_id: u64,
}
