// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SagaSubmitted {
    #[serde(rename = "commandId")]
    pub command_id: String,
}

impl SagaSubmitted {
    pub fn new(command_id: String) -> SagaSubmitted {
        SagaSubmitted { command_id }
    }
}
