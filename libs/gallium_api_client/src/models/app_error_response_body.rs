// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppErrorResponseBody {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    #[serde(rename = "errorId")]
    pub error_id: String,
    #[serde(rename = "errorMap")]
    pub error_map: std::collections::HashMap<String, String>,
    #[serde(rename = "errCode")]
    pub err_code: models::ErrorCode,
}

impl AppErrorResponseBody {
    pub fn new(
        message: String,
        request_id: String,
        status_code: u16,
        error_id: String,
        error_map: std::collections::HashMap<String, String>,
        err_code: models::ErrorCode,
    ) -> AppErrorResponseBody {
        AppErrorResponseBody {
            message,
            request_id,
            status_code,
            error_id,
            error_map,
            err_code,
        }
    }
}
