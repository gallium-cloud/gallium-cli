// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobOutcome {
    #[serde(rename = "Succeeded")]
    Succeeded,
    #[serde(rename = "Failed")]
    Failed,
}

impl std::fmt::Display for JobOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Succeeded => write!(f, "Succeeded"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}

impl Default for JobOutcome {
    fn default() -> JobOutcome {
        Self::Succeeded
    }
}
