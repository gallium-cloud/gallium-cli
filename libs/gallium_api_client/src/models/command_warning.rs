// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandWarning {
    #[serde(rename = "warnCode", skip_serializing_if = "Option::is_none")]
    pub warn_code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CommandWarning {
    pub fn new() -> CommandWarning {
        CommandWarning {
            warn_code: None,
            message: None,
        }
    }
}
