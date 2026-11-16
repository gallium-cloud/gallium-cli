// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VirtualMachineStatus {
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Provisioning")]
    Provisioning,
    #[serde(rename = "Starting")]
    Starting,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Paused")]
    Paused,
    #[serde(rename = "Stopping")]
    Stopping,
    #[serde(rename = "Terminating")]
    Terminating,
    #[serde(rename = "Migrating")]
    Migrating,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Error")]
    Error,
}

impl std::fmt::Display for VirtualMachineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Stopped => write!(f, "Stopped"),
            Self::Provisioning => write!(f, "Provisioning"),
            Self::Starting => write!(f, "Starting"),
            Self::Running => write!(f, "Running"),
            Self::Paused => write!(f, "Paused"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Terminating => write!(f, "Terminating"),
            Self::Migrating => write!(f, "Migrating"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl Default for VirtualMachineStatus {
    fn default() -> VirtualMachineStatus {
        Self::Stopped
    }
}
