use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ManagerNodeRequest {
    Auth {
        token: String,
    },
}

#[derive(Serialize, Deserialize)]
pub enum ManagerNodeResponse {
    Ok,
    Forbidden,
}
