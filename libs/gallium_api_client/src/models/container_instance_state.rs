// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContainerInstanceState {
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for ContainerInstanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Waiting => write!(f, "waiting"),
            Self::Running => write!(f, "running"),
            Self::Terminated => write!(f, "terminated"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for ContainerInstanceState {
    fn default() -> ContainerInstanceState {
        Self::Waiting
    }
}
