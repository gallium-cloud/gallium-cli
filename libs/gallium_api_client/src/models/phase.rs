// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Phase {
    #[serde(rename = "Deploying")]
    Deploying,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Stopping")]
    Stopping,
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Removing")]
    Removing,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Degraded")]
    Degraded,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Deploying => write!(f, "Deploying"),
            Self::Running => write!(f, "Running"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Stopped => write!(f, "Stopped"),
            Self::Removing => write!(f, "Removing"),
            Self::Failed => write!(f, "Failed"),
            Self::Pending => write!(f, "Pending"),
            Self::Degraded => write!(f, "Degraded"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for Phase {
    fn default() -> Phase {
        Self::Deploying
    }
}
