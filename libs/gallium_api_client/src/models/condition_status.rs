// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConditionStatus {
    #[serde(rename = "True")]
    True,
    #[serde(rename = "False")]
    False,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for ConditionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::Unknown => write!(f, "Unknown"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for ConditionStatus {
    fn default() -> ConditionStatus {
        Self::True
    }
}
