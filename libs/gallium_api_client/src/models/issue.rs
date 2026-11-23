// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    #[serde(rename = "severity")]
    pub severity: Box<models::Severity>,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(
        rename = "line",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub line: Option<Option<u32>>,
    #[serde(rename = "message")]
    pub message: String,
}

impl Issue {
    pub fn new(severity: models::Severity, code: String, path: String, message: String) -> Issue {
        Issue {
            severity: Box::new(severity),
            code,
            path,
            line: None,
            message,
        }
    }
}
