// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumOrgWithParent {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "parentOrg", skip_serializing_if = "Option::is_none")]
    pub parent_org: Option<models::GalliumOrg>,
}

impl GalliumOrgWithParent {
    pub fn new(name: String, slug: String) -> GalliumOrgWithParent {
        GalliumOrgWithParent {
            name,
            slug,
            parent_org: None,
        }
    }
}
