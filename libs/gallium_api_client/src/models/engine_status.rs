// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EngineStatus {
    #[serde(rename = "online")]
    pub online: bool,
    #[serde(rename = "status")]
    pub status: models::EngineState,
}

impl EngineStatus {
    pub fn new(online: bool, status: models::EngineState) -> EngineStatus {
        EngineStatus { online, status }
    }
}
