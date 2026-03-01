use crate::api::errors::ApiClientError;
use crate::api::login_api::LoginApi;
use crate::api::vm_service_api::VmServiceApi;
use crate::helpers::auth::AccessToken;
use std::sync::Arc;
use url::Url;

mod common_api;
pub mod errors;
pub(crate) mod login_api;
pub mod vm_service_api;

pub struct ApiClient {
    pub api_url: Url,
    pub access_token: Option<AccessToken>,
}

impl ApiClient {
    pub fn new(api_url: Url) -> Arc<Self> {
        Arc::new(Self {
            api_url,
            access_token: None,
        })
    }

    pub fn with_access_token(&self, access_token: AccessToken) -> Arc<Self> {
        Arc::new(Self {
            api_url: self.api_url.clone(),
            access_token: Some(access_token),
        })
    }

    pub fn get_access_token(&self) -> Result<&AccessToken, ApiClientError> {
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

    pub fn login_api(self: &Arc<Self>) -> LoginApi {
        LoginApi::new(self.clone())
    }

    pub fn vm_service_api(self: &Arc<Self>) -> VmServiceApi {
        VmServiceApi::new(self.clone())
    }
}
