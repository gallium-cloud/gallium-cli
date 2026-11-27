// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogLine {
    #[serde(
        rename = "timestamp",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(rename = "text")]
    pub text: String,
}

impl LogLine {
    pub fn new(text: String) -> LogLine {
        LogLine {
            timestamp: None,
            text,
        }
    }
}
