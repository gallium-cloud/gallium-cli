// =============================================================================
// AUTO-GENERATED — DO NOT EDIT
// =============================================================================

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiCmdStatus {
    PENDING,
    #[serde(rename = "IN_PROGRESS")]
    INPROGRESS,
    COMPLETE,
    FAILED,
}

/// An identifier that uniquely identifies an event, user, resource object, etc in Gallium's platform.
pub type GalliumSlug = String;

/// Response for GetCommandDetails GET /api/v2/command/{id}/details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandApiV2DetailsResponse {
    #[serde(rename = "cmdType")]
    pub cmd_type: String,
    #[serde(rename = "commandSlug")]
    pub command_slug: GalliumSlug,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<serde_json::Value>,
    pub status: ApiCmdStatus,
}

/// Path parameters for GetCommandDetails GET /api/v2/command/{id}/details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommandDetailsPathParams {
    pub id: GalliumSlug,
}

// =============================================================================
// GET /api/v2/command/{id}/details GetCommandDetails
// Get detailed information about specific command
// Security: bearerAuth
// =============================================================================
// (no request body)
