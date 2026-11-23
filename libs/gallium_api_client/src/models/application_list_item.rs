// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationListItem {
    /// An identifier that uniquely identifies an event, user, resource object, etc in Gallium's platform.
    #[serde(rename = "engineSlug")]
    pub engine_slug: String,
    #[serde(rename = "engineName")]
    pub engine_name: String,
    #[serde(rename = "engineOnline")]
    pub engine_online: bool,
    #[serde(rename = "name")]
    pub name: String,
    /// An identifier that uniquely identifies an event, user, resource object, etc in Gallium's platform.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(
        rename = "phase",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase: Option<Option<Box<models::Phase>>>,
    #[serde(
        rename = "observedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub observed_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "syncedAt")]
    pub synced_at: chrono::DateTime<chrono::FixedOffset>,
}

impl ApplicationListItem {
    pub fn new(
        engine_slug: String,
        engine_name: String,
        engine_online: bool,
        name: String,
        application_id: String,
        synced_at: chrono::DateTime<chrono::FixedOffset>,
    ) -> ApplicationListItem {
        ApplicationListItem {
            engine_slug,
            engine_name,
            engine_online,
            name,
            application_id,
            phase: None,
            observed_at: None,
            synced_at,
        }
    }
}
