// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobTrigger {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "deploy")]
    Deploy,
}

impl std::fmt::Display for JobTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Install => write!(f, "install"),
            Self::Deploy => write!(f, "deploy"),
        }
    }
}

impl Default for JobTrigger {
    fn default() -> JobTrigger {
        Self::Install
    }
}
