use crate::api::ApiClient;
use crate::api::common_api::entities::GalliumApiErrorResponse;
use crate::api::errors::ApiClientError;
use crate::api::vm_service_api::entities::{GetWsUrlForVmServiceQueryParams, VncUrlResponse};
use derive_more::Constructor;
use std::sync::Arc;

#[allow(unused)]
pub mod entities;

#[derive(Constructor)]
pub struct VmServiceApi {
    api_client: Arc<ApiClient>,
}

impl VmServiceApi {
    pub async fn get_ws_url_for_vm_service(
        &self,
        params: &GetWsUrlForVmServiceQueryParams,
    ) -> Result<VncUrlResponse, ApiClientError> {
        let response = reqwest::Client::new()
            .get(
                self.api_client
                    .build_url(&["api", "ws", "ws_for_vm_service"])?,
            )
            .query(params)
            .bearer_auth(self.api_client.get_access_token()?.0.as_str())
            .header("Gallium-CLI", clap::crate_version!())
            .send()
            .await?;

        if let Some(msg) = response
            .headers()
            .get("X-Gallium-Cli-Msg")
            .and_then(|h| h.to_str().ok())
        {
            eprintln!("{msg}");
        }

        if response.status().is_success() {
            Ok(response.json::<VncUrlResponse>().await?)
        } else {
            Err(ApiClientError::Api {
                error: response.json::<GalliumApiErrorResponse>().await?,
            })
        }
    }
}
