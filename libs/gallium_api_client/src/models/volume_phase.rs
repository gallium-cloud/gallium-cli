// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumePhase {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Attached")]
    Attached,
    #[serde(rename = "Lost")]
    Lost,
    #[serde(rename = "Orphaned")]
    Orphaned,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for VolumePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Attached => write!(f, "Attached"),
            Self::Lost => write!(f, "Lost"),
            Self::Orphaned => write!(f, "Orphaned"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for VolumePhase {
    fn default() -> VolumePhase {
        Self::Pending
    }
}
