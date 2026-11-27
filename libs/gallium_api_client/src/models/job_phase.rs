// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobPhase {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Succeeded")]
    Succeeded,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for JobPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Running => write!(f, "Running"),
            Self::Succeeded => write!(f, "Succeeded"),
            Self::Failed => write!(f, "Failed"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for JobPhase {
    fn default() -> JobPhase {
        Self::Pending
    }
}
