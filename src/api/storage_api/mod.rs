use crate::api::errors::ApiClientError;

use crate::api::ApiClient;
use crate::api::storage_api::entities::{DiskPoolListResponse, ListDiskPoolsPathParams};
use derive_more::Constructor;
use reqwest::Method;
use std::sync::Arc;

pub mod entities;

#[derive(Constructor)]
pub struct StorageApi {
    api_client: Arc<ApiClient>,
}

impl StorageApi {
    pub async fn list_disk_pools(
        &self,
        path_params: &ListDiskPoolsPathParams,
    ) -> Result<DiskPoolListResponse, ApiClientError> {
        let response = self
            .api_client
            .request_authed(
                Method::GET,
                &["cluster-api", &path_params.cluster_id, "storage-class"],
            )?
            .send()
            .await?;

        self.api_client.deser_response(response).await
    }
}
