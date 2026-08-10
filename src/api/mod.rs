use crate::api::common_api::entities::GalliumApiErrorResponse;
use crate::api::errors::{
    ApiClientError, DecodeErrorResponseSnafu, DecodeResponseSnafu, ReadResponseBodySnafu,
    ResponseContext,
};

use crate::api::cluster_vm_api::ClusterVmApi;
use crate::api::command_v2_api::CommandApi;
use crate::api::login_api::LoginApi;
use crate::api::storage_api::StorageApi;
use crate::api::vm_service_api::VmServiceApi;
use crate::helpers::auth::AccessToken;
use reqwest::header;
use serde::de::DeserializeOwned;
use snafu::ResultExt;
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

        let status = response.status();
        let url = response.url().clone();
        let bytes = response.bytes().await.context(ReadResponseBodySnafu {
            status,
            url: Box::new(url.clone()),
        })?;

        if status.is_success() {
            serde_json::from_slice::<T>(&bytes).context(DecodeResponseSnafu {
                ctx: Box::new(ResponseContext {
                    status,
                    url,
                    body: truncate_body(&bytes),
                }),
            })
        } else {
            let error = serde_json::from_slice::<GalliumApiErrorResponse>(&bytes).context(
                DecodeErrorResponseSnafu {
                    ctx: Box::new(ResponseContext {
                        status,
                        url: url.clone(),
                        body: truncate_body(&bytes),
                    }),
                },
            )?;
            Err(ApiClientError::Api {
                status,
                url,
                error: Box::new(error),
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

fn truncate_body(bytes: &[u8]) -> String {
    const MAX: usize = 2048;
    let s = String::from_utf8_lossy(bytes);
    if s.len() <= MAX {
        return s.into_owned();
    }
    let truncated = unicode_ellipsis::truncate_str(&s, MAX);
    format!("{truncated} ({} bytes total)", bytes.len())
}
