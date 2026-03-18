use crate::api::common_api::entities::GalliumApiErrorResponse;
use crate::api::errors::ApiClientError;

use crate::api::cluster_vm_api::ClusterVmApi;
use crate::api::command_v2_api::CommandApi;
use crate::api::login_api::LoginApi;
use crate::api::storage_api::StorageApi;
use crate::api::vm_service_api::VmServiceApi;
use crate::helpers::auth::AccessToken;
use reqwest::header;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use url::Url;

pub mod cluster_vm_api;
pub mod command_v2_api;
mod common_api;
pub mod errors;
pub(crate) mod login_api;
pub mod storage_api;
pub mod vm_service_api;

pub struct ApiClient {
    pub api_url: Url,
    pub access_token: Option<AccessToken>,
    pub client: reqwest::Client,
}

impl ApiClient {
    pub fn new(api_url: Url) -> Arc<Self> {
        Arc::new(Self {
            api_url,
            access_token: None,
            client: reqwest::Client::new(),
        })
    }

    pub fn with_access_token(&self, access_token: AccessToken) -> Arc<Self> {
        Arc::new(Self {
            api_url: self.api_url.clone(),
            access_token: Some(access_token),
            client: self.client.clone(),
        })
    }

    fn get_access_token(&self) -> Result<&AccessToken, ApiClientError> {
        self.access_token
            .as_ref()
            .ok_or_else(|| ApiClientError::InternalError {
                reason: "access token missing",
            })
    }

    fn build_url(&self, segments_in: &[&str]) -> Result<Url, ApiClientError> {
        let mut url = self.api_url.clone();
        let mut segments_out =
            url.path_segments_mut()
                .map_err(|()| ApiClientError::InternalError {
                    reason: "invalid base URL",
                })?;
        for segment in segments_in.iter() {
            if segment.starts_with(".") {
                return Err(ApiClientError::InvalidPathSegmentParameter {
                    val: segment.to_string(),
                });
            }
            segments_out.push(segment);
        }
        drop(segments_out);

        Ok(url)
    }

    fn request(
        &self,
        method: reqwest::Method,
        segments_in: &[&str],
    ) -> Result<reqwest::RequestBuilder, ApiClientError> {
        Ok(self
            .client
            .request(method, self.build_url(segments_in)?)
            .header("Gallium-CLI", clap::crate_version!())
            .header(
                header::USER_AGENT,
                "Gallium Cloud CLI (https://gallium.cloud)",
            ))
    }
    fn request_authed(
        &self,
        method: reqwest::Method,
        segments_in: &[&str],
    ) -> Result<reqwest::RequestBuilder, ApiClientError> {
        Ok(self
            .request(method, segments_in)?
            .bearer_auth(self.get_access_token()?.0.as_str()))
    }

    async fn deser_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, ApiClientError> {
        if let Some(msg) = response
            .headers()
            .get("X-Gallium-Cli-Msg")
            .and_then(|h| h.to_str().ok())
        {
            eprintln!("{msg}");
        }

        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            Err(ApiClientError::Api {
                error: response.json::<GalliumApiErrorResponse>().await?,
            })
        }
    }

    pub fn cluster_vm_api(self: &Arc<Self>) -> ClusterVmApi {
        ClusterVmApi::new(self.clone())
    }

    pub fn login_api(self: &Arc<Self>) -> LoginApi {
        LoginApi::new(self.clone())
    }

    pub fn vm_service_api(self: &Arc<Self>) -> VmServiceApi {
        VmServiceApi::new(self.clone())
    }

    pub fn storage_api(self: &Arc<Self>) -> StorageApi {
        StorageApi::new(self.clone())
    }

    pub fn command_api(self: &Arc<Self>) -> CommandApi {
        CommandApi::new(self.clone())
    }
}
