// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandApiV2ProgressPutRequest {
    #[serde(
        rename = "status",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<Option<models::ApiCmdStatus>>,
    #[serde(
        rename = "progressCurrent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub progress_current: Option<Option<f64>>,
    #[serde(
        rename = "progressTotal",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub progress_total: Option<Option<f64>>,
    #[serde(
        rename = "progressMessage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub progress_message: Option<Option<String>>,
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
