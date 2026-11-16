// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumOrg {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl GalliumOrg {
    pub fn new(name: String, slug: String) -> GalliumOrg {
        GalliumOrg { name, slug }
    }
}
