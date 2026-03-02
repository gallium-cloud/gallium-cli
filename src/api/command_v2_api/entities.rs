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

// =============================================================================
// PUT /api/v2/command/{id}/progress UpdateCommandProgress
// Update the backend with information about the progress of an externally managed command
// Content-Type: application/json
// Security: bearerAuth
// =============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandApiV2ProgressPutRequest {
    #[serde(rename = "progressCurrent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_current: Option<f64>,
    #[serde(rename = "progressMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_message: Option<String>,
    #[serde(rename = "progressTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ApiCmdStatus>,
}

/// Response for UpdateCommandProgress PUT /api/v2/command/{id}/progress
pub type CommandApiV2ProgressResponse = serde_json::Value;

/// Path parameters for GetCommandDetails GET /api/v2/command/{id}/details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommandDetailsPathParams {
    pub id: GalliumSlug,
}

/// Path parameters for UpdateCommandProgress PUT /api/v2/command/{id}/progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommandProgressPathParams {
    pub id: GalliumSlug,
}

// =============================================================================
// GET /api/v2/command/{id}/details GetCommandDetails
// Get detailed information about specific command
// Security: bearerAuth
// =============================================================================
// (no request body)
