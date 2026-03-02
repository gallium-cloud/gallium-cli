use crate::api::errors::ApiClientError;

use crate::api::ApiClient;
use crate::api::storage_api::entities::{
    CmdSubmitResponse, DiskPoolListResponse, ImportNbdVolumePathParams, ListDiskPoolsPathParams,
    VolumeNbdImportRequest,
};
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
    pub async fn import_nbd_volume(
        &self,
        path_params: &ImportNbdVolumePathParams,
        request_body: &VolumeNbdImportRequest,
    ) -> Result<CmdSubmitResponse, ApiClientError> {
        let response = self
            .api_client
            .request_authed(
                Method::POST,
                &[
                    "cluster-api",
                    &path_params.cluster_id,
                    "volume",
                    &path_params.kube_ns,
                    "nbd",
                    "import",
                ],
            )?
            .json(request_body)
            .send()
            .await?;
        self.api_client.deser_response(response).await
    }
}
