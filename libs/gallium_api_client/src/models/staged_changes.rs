// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StagedChanges {
    #[serde(rename = "any")]
    pub any: bool,
    #[serde(rename = "document")]
    pub document: bool,
    #[serde(rename = "inputs")]
    pub inputs: models::ChangeCounts,
    #[serde(rename = "files")]
    pub files: models::ChangeCounts,
    #[serde(rename = "ingresses")]
    pub ingresses: models::ChangeCounts,
}

impl StagedChanges {
    pub fn new(
        any: bool,
        document: bool,
        inputs: models::ChangeCounts,
        files: models::ChangeCounts,
        ingresses: models::ChangeCounts,
    ) -> StagedChanges {
        StagedChanges {
            any,
            document,
            inputs,
            files,
            ingresses,
        }
    }
}
