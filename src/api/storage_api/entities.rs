// =============================================================================
// AUTO-GENERATED — DO NOT EDIT
// =============================================================================

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdEntity {
    pub entity_type: String,
    pub kube_name: String,
    pub kube_ns: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubCommand {
    #[serde(rename = "cmdType")]
    pub cmd_type: String,
    #[serde(rename = "commandSlug")]
    pub command_slug: String,
}

/// Response for ImportNbdVolume POST /cluster-api/{cluster_id}/volume/{kube_ns}/nbd/import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdSubmitResponse {
    #[serde(rename = "cmdType")]
    pub cmd_type: String,
    #[serde(rename = "commandSlug")]
    pub command_slug: String,
    pub message: String,
    #[serde(rename = "subCommands")]
    pub sub_commands: Vec<SubCommand>,
    #[serde(rename = "targetEntity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_entity: Option<CmdEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskPoolSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isUserDefault")]
    pub is_user_default: bool,
    #[serde(rename = "kubeName")]
    pub kube_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Response for ListDiskPools GET /cluster-api/{cluster_id}/storage-class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskPoolListResponse {
    pub pools: Vec<DiskPoolSummary>,
}

/// An identifier that uniquely identifies an event, user, resource object, etc in Gallium's platform.
pub type GalliumSlug = String;

/// Path parameters for ExportNbdVolume POST /cluster-api/{cluster_id}/volume/{kube_ns}/{kube_name}/nbd/export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportNbdVolumePathParams {
    pub cluster_id: GalliumSlug,
    pub kube_name: String,
    pub kube_ns: String,
}

/// Path parameters for ImportNbdVolume POST /cluster-api/{cluster_id}/volume/{kube_ns}/nbd/import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportNbdVolumePathParams {
    pub cluster_id: GalliumSlug,
    pub kube_ns: String,
}

/// Path parameters for ListDiskPools GET /cluster-api/{cluster_id}/storage-class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDiskPoolsPathParams {
    pub cluster_id: GalliumSlug,
}

/// Path parameters for ListVolumes GET /cluster-api/{cluster_id}/volume/{kube_ns}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVolumesPathParams {
    pub cluster_id: GalliumSlug,
    pub kube_ns: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeTypeLabel {
    TemplateImage,
    Iso,
    UserStorage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kubeName")]
    pub kube_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The total size of the volume
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    /// The amount of data that is actually allocated within the volume (actualSize in longhorn terminology). NOTE: Space free-ed by the OS may not be reliably reclaimed.
    #[serde(rename = "usedSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_size_bytes: Option<u64>,
    #[serde(rename = "volumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<VolumeTypeLabel>,
}

/// Response for ListVolumes GET /cluster-api/{cluster_id}/volume/{kube_ns}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeListResponse {
    pub volumes: Vec<Volume>,
}

// =============================================================================
// POST /cluster-api/{cluster_id}/volume/{kube_ns}/{kube_name}/nbd/export ExportNbdVolume
// Command to export a volume via NBD
// Content-Type: application/json
// Security: bearerAuth
// =============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeNbdExportRequest {
    #[serde(rename = "csrBase64")]
    pub csr_base64: String,
}

// =============================================================================
// POST /cluster-api/{cluster_id}/volume/{kube_ns}/nbd/import ImportNbdVolume
// Command to import a volume via NBD
// Content-Type: application/json
// Security: bearerAuth
// =============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeNbdImportRequest {
    #[serde(rename = "csrBase64")]
    pub csr_base64: String,
    #[serde(rename = "importSourceFileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source_file_name: Option<String>,
    #[serde(rename = "volumeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_description: Option<String>,
    #[serde(rename = "volumeSizeGb")]
    pub volume_size_gb: u32,
    #[serde(rename = "volumeStorageClass")]
    pub volume_storage_class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}

// =============================================================================
// GET /cluster-api/{cluster_id}/storage-class ListDiskPools
// List existing disk pools
// Security: bearerAuth
// =============================================================================
// (no request body)

// =============================================================================
// GET /cluster-api/{cluster_id}/volume/{kube_ns} ListVolumes
// List existing volumes
// Security: bearerAuth
// =============================================================================
// (no request body)
