// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoolKind {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

impl std::fmt::Display for PoolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::System => write!(f, "system"),
            Self::User => write!(f, "user"),
        }
    }
}

impl Default for PoolKind {
    fn default() -> PoolKind {
        Self::All
    }
}
