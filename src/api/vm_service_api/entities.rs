// =============================================================================
// AUTO-GENERATED — DO NOT EDIT
// =============================================================================

use serde::{Deserialize, Serialize};

/// Query parameters for getWsUrlForVmService GET /api/ws/ws_for_vm_service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWsUrlForVmServiceQueryParams {
    /// The port number to connect to on the VM
    pub port: String,
    /// The name or slug of the VM to connect to
    pub host: String,
}

/// Response for getWsUrlForVmService GET /api/ws/ws_for_vm_service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncUrlResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// =============================================================================
// GET /api/ws/ws_for_vm_service getWsUrlForVmService
// Get Websocket URL to a service on the VM (only supported for NAT networks)
// Security: bearerAuth
// =============================================================================
// (no request body)
