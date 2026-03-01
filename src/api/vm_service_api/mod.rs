use crate::api::ApiClient;
use crate::api::common_api::entities::GalliumApiErrorResponse;
use crate::api::errors::ApiClientError;
use crate::api::vm_service_api::entities::{GetWsUrlForVmServiceQueryParams, VncUrlResponse};
use derive_more::Constructor;
use reqwest::Method;
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
        let response = self
            .api_client
            .request_authed(Method::GET, &["api", "ws", "ws_for_vm_service"])?
            .query(params)
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
