// =============================================================================
// AUTO-GENERATED — DO NOT EDIT
// =============================================================================

use serde::{Deserialize, Serialize};

pub type Ipv4Network = String;

/// If dhcp is enabled, ip and gateway must be null. If dhcp is disabled, ip and gateway must be provided and valid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressConfig {
    pub dhcp: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Ipv4Network>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfiguration {
    pub nameservers: Vec<String>,
}

/// An identifier that uniquely identifies an event, user, resource object, etc in Gallium's platform.
pub type GalliumSlug = String;

/// Path parameters for ListVirtualMachines GET /cluster-api/{cluster_id}/vm/{kube_ns}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVirtualMachinesPathParams {
    pub cluster_id: GalliumSlug,
    pub kube_ns: String,
}

/// MAC address as a string
pub type MacAddress = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmNetworkInterfaceConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DnsConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<AddressConfig>,
    #[serde(rename = "macAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_addr: Option<MacAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VirtualMachineStatus {
    Stopped,
    Provisioning,
    Starting,
    Running,
    Paused,
    Stopping,
    Terminating,
    Migrating,
    Unknown,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineVolume {
    pub bootable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
    #[serde(rename = "volumeKubeName")]
    pub volume_kube_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub interfaces: Vec<VmNetworkInterfaceConfiguration>,
    #[serde(rename = "kubeName")]
    pub kube_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub status: VirtualMachineStatus,
    pub volumes: Vec<VirtualMachineVolume>,
}

/// Response for ListVirtualMachines GET /cluster-api/{cluster_id}/vm/{kube_ns}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineListResponse {
    pub items: Vec<VirtualMachine>,
}

// =============================================================================
// GET /cluster-api/{cluster_id}/vm/{kube_ns} ListVirtualMachines
// List existing virtual machines
// Security: bearerAuth
// =============================================================================
// (no request body)
