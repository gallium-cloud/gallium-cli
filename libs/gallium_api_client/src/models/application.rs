// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "metadata")]
    pub metadata: Box<models::ApplicationMetadata>,
    #[serde(rename = "status")]
    pub status: Box<models::ApplicationStatus>,
}

impl Application {
    pub fn new(
        metadata: models::ApplicationMetadata,
        status: models::ApplicationStatus,
    ) -> Application {
        Application {
            metadata: Box::new(metadata),
            status: Box::new(status),
        }
    }
}
