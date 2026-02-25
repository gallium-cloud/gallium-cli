// =============================================================================
// AUTO-GENERATED — DO NOT EDIT
// =============================================================================

use serde::{Deserialize, Serialize};

/// Response for invalidateRefreshToken POST /api/token/invalidate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalliumApiSuccessResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// =============================================================================
// POST /api/login login
// Login
// Content-Type: application/json
// =============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalliumLoginRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp: Option<String>,
    pub password: String,
    /// The client's refresh token, if known. If provided here, it will be invalidated and a new one will be issued.
    #[serde(rename = "refreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalliumOrg {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalliumOrgWithParent {
    pub name: String,
    #[serde(rename = "parentOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org: Option<GalliumOrg>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalliumUser {
    #[serde(rename = "darkMode")]
    pub dark_mode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Response for login POST /api/login
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalliumLoginResponse {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "availableOrgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_orgs: Option<Vec<GalliumOrgWithParent>>,
    #[serde(rename = "mfaRequired")]
    pub mfa_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<GalliumOrgWithParent>,
    #[serde(rename = "refreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<GalliumUser>,
}

// =============================================================================
// POST /api/token refreshAccessToken
// Get a fresh access token using the refresh token.
// Content-Type: application/json
// =============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalliumTokenRequest {
    #[serde(rename = "orgSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_slug: Option<String>,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

// =============================================================================
// POST /api/token/invalidate invalidateRefreshToken
// Mark a refresh token as invalid
// Content-Type: application/json
// =============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidateTokenRequest {
    /// Whether to invalidate all tokens for this user. If true, all tokens will be invalidated. Otherwise, only the token supplied in refreshToken will be invalidated.
    #[serde(rename = "invalidateAll")]
    pub invalidate_all: bool,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}
