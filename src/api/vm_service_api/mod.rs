use crate::api::vm_service_api::entities::{GetWsUrlForVmServiceQueryParams, VncUrlResponse};
use crate::api::ApiClient;
use anyhow::anyhow;
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
        access_token: impl ToString,
        params: &GetWsUrlForVmServiceQueryParams,
    ) -> anyhow::Result<String> {
        let response = reqwest::Client::new()
            .get(self.api_client.api_url.join("/api/ws/ws_for_vm_service")?)
            .query(params)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", access_token.to_string()),
            )
            .header("Gallium-CLI", clap::crate_version!())
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(response.text().await.unwrap());
        }

        if let Some(msg) = response.headers().get("X-Gallium-Cli-Msg") {
            eprintln!(
                "{}",
                std::str::from_utf8(msg.as_bytes()).expect("utf-8 msg header")
            );
        }

        response
            .json::<VncUrlResponse>()
            .await?
            .url
            .ok_or_else(|| anyhow!("get_ws_url response missing url"))
    }
}
