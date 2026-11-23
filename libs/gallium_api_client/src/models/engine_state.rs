// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EngineState {
    String(String),
    String2(String),
    String3(String),
    String4(String),
    String5(String),
    String6(String),
}

impl Default for EngineState {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
