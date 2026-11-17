// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CliTokenRequest {
    /// Authorization code from the console.
    #[serde(rename = "code")]
    pub code: String,
    /// PKCE code verifier whose S256 challenge was sent when the code was requested (43 to 128 unreserved characters).
    #[serde(rename = "codeVerifier")]
    pub code_verifier: String,
}

impl CliTokenRequest {
    pub fn new(code: String, code_verifier: String) -> CliTokenRequest {
        CliTokenRequest {
            code,
            code_verifier,
        }
    }
}
