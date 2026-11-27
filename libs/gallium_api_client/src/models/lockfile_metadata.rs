// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LockfileMetadata {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "sequence")]
    pub sequence: u32,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl LockfileMetadata {
    pub fn new(
        id: String,
        sequence: u32,
        created_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> LockfileMetadata {
        LockfileMetadata {
            id,
            sequence,
            created_at,
        }
    }
}
