use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenWalletRequest {
    pub filename: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<String>,
}