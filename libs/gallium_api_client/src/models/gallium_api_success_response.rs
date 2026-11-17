// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumApiSuccessResponse {
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<models::CommandWarning>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(
        rename = "command",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub command: Option<Option<Box<models::CommandSummary>>>,
}

impl GalliumApiSuccessResponse {
    pub fn new() -> GalliumApiSuccessResponse {
        GalliumApiSuccessResponse {
            warnings: None,
            message: None,
            command: None,
        }
    }
}
