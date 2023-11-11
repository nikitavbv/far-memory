use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ManagerNodeRequest {
    Auth {
        token: String,
    },
    GetReplacementPolicyParams(ReplacementPolicyType),
    SpanAccessStats(Vec<SpanAccessEvent>),
    FinishSession,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ManagerNodeResponse {
    Ok,
    Forbidden,
    ReplacementPolicyParams {
        span_access_history: Option<Vec<SpanAccessEvent>>,
    }
}

#[derive(Serialize, Deserialize)]
pub enum ReplacementPolicyType {
    Replay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpanAccessEvent {
    pub time_step: u64,
    pub span_id: u64,
}
