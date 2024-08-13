use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Request failed with status: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("Request failed with error: {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("OpenAI API returned an error: {0:?}")]
    ApiError(Error),
}
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
    pub param: Option<String>,
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: String,
}
