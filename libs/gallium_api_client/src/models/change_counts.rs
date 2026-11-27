// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeCounts {
    #[serde(rename = "new")]
    pub new: u32,
    #[serde(rename = "updated")]
    pub updated: u32,
    #[serde(rename = "removed")]
    pub removed: u32,
}

impl ChangeCounts {
    pub fn new(new: u32, updated: u32, removed: u32) -> ChangeCounts {
        ChangeCounts {
            new,
            updated,
            removed,
        }
    }
}
