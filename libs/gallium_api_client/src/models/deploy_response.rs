// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployResponse {
    #[serde(rename = "commandId")]
    pub command_id: String,
}

impl DeployResponse {
    pub fn new(command_id: String) -> DeployResponse {
        DeployResponse { command_id }
    }
}
