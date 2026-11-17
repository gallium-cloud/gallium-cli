// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvalidateTokenRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// Whether to invalidate all tokens for this user. If true, all tokens will be invalidated. Otherwise, only the token supplied in refreshToken will be invalidated.
    #[serde(rename = "invalidateAll")]
    pub invalidate_all: bool,
}

impl InvalidateTokenRequest {
    pub fn new(refresh_token: String, invalidate_all: bool) -> InvalidateTokenRequest {
        InvalidateTokenRequest {
            refresh_token,
            invalidate_all,
        }
    }
}
