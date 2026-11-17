// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumApiErrorResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, String>>,
    #[serde(
        rename = "error",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<Option<String>>,
    #[serde(rename = "errCode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<String>,
    #[serde(rename = "errorId", skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
}

impl GalliumApiErrorResponse {
    pub fn new() -> GalliumApiErrorResponse {
        GalliumApiErrorResponse {
            errors: None,
            error: None,
            err_code: None,
            error_id: None,
        }
    }
}
