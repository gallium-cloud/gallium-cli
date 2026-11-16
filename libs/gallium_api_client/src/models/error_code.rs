// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

/// ErrorCode : Citations for error code descriptions: * https://github.com/googleapis/googleapis/blob/master/google/rpc/code.proto * https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go * https://www.rfc-editor.org/rfc/rfc6750.html#section-3.1 * https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.10
/// Citations for error code descriptions: * https://github.com/googleapis/googleapis/blob/master/google/rpc/code.proto * https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go * https://www.rfc-editor.org/rfc/rfc6750.html#section-3.1 * https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.10
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorCode {
    /// An unexpected server-side failure prevented the request from completing, and its outcome is undetermined.
    String(String),
    /// A request parameter is malformed or invalid, independent of the current state of the system.
    String2(String),
    /// The requested resource does not exist.
    String3(String),
    /// The request did not carry valid authentication credentials, so the caller could not be identified.
    String4(String),
    /// The access token presented with the request has expired; obtain a new token and retry.
    String5(String),
    /// The request conflicts with the current state of the target resource and must be altered before retrying.
    String6(String),
    /// The caller was identified but is not permitted to perform this operation.
    String7(String),
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
