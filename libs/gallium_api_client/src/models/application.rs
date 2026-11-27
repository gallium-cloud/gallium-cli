// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "metadata")]
    pub metadata: models::ApplicationMetadata,
    #[serde(rename = "status")]
    pub status: models::ApplicationStatus,
}

impl Application {
    pub fn new(
        metadata: models::ApplicationMetadata,
        status: models::ApplicationStatus,
    ) -> Application {
        Application { metadata, status }
    }
}
