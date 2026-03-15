use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuInfo {
    #[serde(rename = "virtual-size")]
    pub virtual_size: u64,
    pub filename: String,
    #[serde(rename = "cluster-size", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<u64>,
    pub format: String,
    #[serde(rename = "actual-size")]
    pub actual_size: u64,
    #[serde(rename = "dirty-flag")]
    pub dirty_flag: Option<bool>,
    pub children: Vec<Box<QemuInfoChild>>,
    #[serde(rename = "format-specific", skip_serializing_if = "Option::is_none")]
    pub format_specific: Option<QemuInfoFormatSpecific>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuInfoChild {
    pub name: String,
    pub info: Box<QemuInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuBitmap {
    pub flags: Option<Vec<String>>,
    pub name: Option<String>,
    pub granularity: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuInfoFormatSpecificDataQcow2 {
    pub compat: Option<String>,
    #[serde(rename = "compression-type")]
    pub compression_type: Option<String>,
    #[serde(rename = "lazy-refcounts")]
    pub lazy_refcounts: Option<bool>,
    #[serde(rename = "refcount-bits")]
    pub refcount_bits: Option<u16>,
    #[serde(rename = "corrupt")]
    pub corrupt: Option<bool>,
    #[serde(rename = "extended-l2")]
    pub extended_l2: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitmaps: Option<Vec<QemuBitmap>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuInfoVmdkExtent {
    #[serde(rename = "virtual-size")]
    pub virtual_size: u64,
    pub filename: String,
    #[serde(rename = "cluster-size", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<u64>,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuInfoFormatSpecificDataVmdk {
    pub cid: Option<i64>,
    #[serde(rename = "parent-cid")]
    pub parent_cid: Option<i64>,
    #[serde(rename = "create-type")]
    pub create_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extents: Option<Vec<QemuInfoVmdkExtent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QemuInfoFormatSpecificDataFile {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum QemuInfoFormatSpecific {
    #[serde(rename = "qcow2")]
    Qcow2(QemuInfoFormatSpecificDataQcow2),
    #[serde(rename = "vmdk")]
    Vmdk(QemuInfoFormatSpecificDataVmdk),
    #[serde(rename = "file")]
    File(QemuInfoFormatSpecificDataFile),
}
