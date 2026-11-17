// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumTokenRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(
        rename = "orgSlug",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub org_slug: Option<Option<String>>,
}

impl GalliumTokenRequest {
    pub fn new(refresh_token: String) -> GalliumTokenRequest {
        GalliumTokenRequest {
            refresh_token,
            org_slug: None,
        }
    }
}
