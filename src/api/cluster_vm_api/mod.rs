use crate::api::ApiClient;
use crate::api::cluster_vm_api::entities::{
    ListVirtualMachinesPathParams, VirtualMachineListResponse,
};
use crate::api::errors::ApiClientError;
use derive_more::Constructor;
use reqwest::Method;
use std::sync::Arc;

#[allow(unused)]
pub mod entities;
#[derive(Constructor)]
pub struct ClusterVmApi {
    api_client: Arc<ApiClient>,
}

impl ClusterVmApi {
    pub async fn list_virtual_machines(
        &self,
        path_params: &ListVirtualMachinesPathParams,
    ) -> Result<VirtualMachineListResponse, ApiClientError> {
        let response = self
            .api_client
            .request_authed(
                Method::GET,
                &[
                    "cluster-api",
                    &path_params.cluster_id,
                    "vm",
                    &path_params.kube_ns,
                ],
            )?
            .send()
            .await?;

        self.api_client.deser_response(response).await
    }
}
