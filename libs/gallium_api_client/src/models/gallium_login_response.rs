// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumLoginResponse {
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "org", skip_serializing_if = "Option::is_none")]
    pub org: Option<models::GalliumOrgWithParent>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<models::GalliumUser>,
    #[serde(rename = "availableOrgs", skip_serializing_if = "Option::is_none")]
    pub available_orgs: Option<Vec<models::GalliumOrgWithParent>>,
    #[serde(rename = "mfaRequired")]
    pub mfa_required: bool,
}

impl GalliumLoginResponse {
    pub fn new(mfa_required: bool) -> GalliumLoginResponse {
        GalliumLoginResponse {
            access_token: None,
            refresh_token: None,
            org: None,
            user: None,
            available_orgs: None,
            mfa_required,
        }
    }
}
