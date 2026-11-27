// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationMetadata {
    #[serde(rename = "name")]
    pub name: String,
    /// An identifier that uniquely identifies an event, user, resource object, etc in Gallium's platform.
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ApplicationMetadata {
    pub fn new(
        name: String,
        created_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> ApplicationMetadata {
        ApplicationMetadata {
            name,
            application_id: None,
            created_at,
        }
    }
}
