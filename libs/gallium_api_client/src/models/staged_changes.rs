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
    pub inputs: Box<models::ChangeCounts>,
    #[serde(rename = "files")]
    pub files: Box<models::ChangeCounts>,
    #[serde(rename = "ingresses")]
    pub ingresses: Box<models::ChangeCounts>,
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
            inputs: Box::new(inputs),
            files: Box::new(files),
            ingresses: Box::new(ingresses),
        }
    }
}
