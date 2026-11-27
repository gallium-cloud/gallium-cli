// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EngineState {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unhealthy")]
    Unhealthy,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "notInstalled")]
    NotInstalled,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for EngineState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Connected => write!(f, "connected"),
            Self::Initializing => write!(f, "initializing"),
            Self::Unsupported => write!(f, "unsupported"),
            Self::Unhealthy => write!(f, "unhealthy"),
            Self::Offline => write!(f, "offline"),
            Self::NotInstalled => write!(f, "notInstalled"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for EngineState {
    fn default() -> EngineState {
        Self::Connected
    }
}
