// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiCmdStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for ApiCmdStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::InProgress => write!(f, "IN_PROGRESS"),
            Self::Complete => write!(f, "COMPLETE"),
            Self::Failed => write!(f, "FAILED"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for ApiCmdStatus {
    fn default() -> ApiCmdStatus {
        Self::Pending
    }
}
