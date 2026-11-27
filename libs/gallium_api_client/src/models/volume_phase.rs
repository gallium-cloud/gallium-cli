// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumePhase {
    String(String),
    String2(String),
    String3(String),
    String4(String),
}

impl Default for VolumePhase {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
