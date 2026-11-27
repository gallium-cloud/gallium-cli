// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandApiV2ProgressPutRequest {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ApiCmdStatus>,
    #[serde(rename = "progressCurrent", skip_serializing_if = "Option::is_none")]
    pub progress_current: Option<f64>,
    #[serde(rename = "progressTotal", skip_serializing_if = "Option::is_none")]
    pub progress_total: Option<f64>,
    #[serde(rename = "progressMessage", skip_serializing_if = "Option::is_none")]
    pub progress_message: Option<String>,
}

impl CommandApiV2ProgressPutRequest {
    pub fn new() -> CommandApiV2ProgressPutRequest {
        CommandApiV2ProgressPutRequest {
            status: None,
            progress_current: None,
            progress_total: None,
            progress_message: None,
        }
    }
}
