// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

/// ErrorCode : Citations for error code descriptions: * https://github.com/googleapis/googleapis/blob/master/google/rpc/code.proto * https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go * https://www.rfc-editor.org/rfc/rfc6750.html#section-3.1 * https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.10
/// Citations for error code descriptions: * https://github.com/googleapis/googleapis/blob/master/google/rpc/code.proto * https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go * https://www.rfc-editor.org/rfc/rfc6750.html#section-3.1 * https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.10
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "INVALID_PARAM")]
    InvalidParam,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "AUTHENTICATION_FAILED")]
    AuthenticationFailed,
    #[serde(rename = "TOKEN_EXPIRED")]
    TokenExpired,
    #[serde(rename = "PARAM_CONFLICT")]
    ParamConflict,
    #[serde(rename = "FORBIDDEN")]
    Forbidden,
    #[serde(untagged)]
    UnknownValue(String),
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "INTERNAL_ERROR"),
            Self::InvalidParam => write!(f, "INVALID_PARAM"),
            Self::NotFound => write!(f, "NOT_FOUND"),
            Self::AuthenticationFailed => write!(f, "AUTHENTICATION_FAILED"),
            Self::TokenExpired => write!(f, "TOKEN_EXPIRED"),
            Self::ParamConflict => write!(f, "PARAM_CONFLICT"),
            Self::Forbidden => write!(f, "FORBIDDEN"),
            Self::UnknownValue(value) => write!(f, "{value}"),
        }
    }
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        Self::InternalError
    }
}
