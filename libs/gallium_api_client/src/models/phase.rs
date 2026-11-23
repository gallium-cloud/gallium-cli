// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Phase {
    String(String),
    String2(String),
    String3(String),
}

impl Default for Phase {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
