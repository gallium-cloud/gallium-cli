// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumLoginRequest {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "otp", skip_serializing_if = "Option::is_none")]
    pub otp: Option<String>,
}

impl GalliumLoginRequest {
    pub fn new(email: String, password: String) -> GalliumLoginRequest {
        GalliumLoginRequest {
            email,
            password,
            otp: None,
        }
    }
}
