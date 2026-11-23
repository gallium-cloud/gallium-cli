// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Severity {
    String(String),
    String2(String),
}

impl Default for Severity {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
