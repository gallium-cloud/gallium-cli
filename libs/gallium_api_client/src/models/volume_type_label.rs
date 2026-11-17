// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeTypeLabel {
    #[serde(rename = "TemplateImage")]
    TemplateImage,
    #[serde(rename = "Iso")]
    Iso,
    #[serde(rename = "UserStorage")]
    UserStorage,
    #[serde(rename = "System")]
    System,
}

impl std::fmt::Display for VolumeTypeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TemplateImage => write!(f, "TemplateImage"),
            Self::Iso => write!(f, "Iso"),
            Self::UserStorage => write!(f, "UserStorage"),
            Self::System => write!(f, "System"),
        }
    }
}

impl Default for VolumeTypeLabel {
    fn default() -> VolumeTypeLabel {
        Self::TemplateImage
    }
}
