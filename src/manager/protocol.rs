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
    ReplacementPolicyParams(ReplacementPolicyParams),
}

#[derive(Serialize, Deserialize)]
pub enum ReplacementPolicyType {
    Replay,
    RNN,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpanAccessEvent {
    pub time_step: u64,
    pub span_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplacementPolicyParams {
    pub span_access_history: Option<Vec<SpanAccessEvent>>,
    pub rnn_weights: Option<RNNWeights>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RNNWeights {
    pub total_spans: u64,
    pub weights: Vec<u8>,
}
