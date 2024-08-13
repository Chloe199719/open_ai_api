use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error: ErrorData,
}
#[derive(Debug)]
pub enum ErrorType {
    ResponseError(Error),
    ReqwestError(reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    pub param: String,
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: String,
}
