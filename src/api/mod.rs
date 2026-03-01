use crate::api::errors::ApiClientError;
use crate::api::login_api::LoginApi;
use crate::api::vm_service_api::VmServiceApi;
use std::sync::Arc;
use url::Url;

mod common_api;
pub mod errors;
pub(crate) mod login_api;
pub mod vm_service_api;

pub struct ApiClient {
    pub api_url: Url,
}

impl ApiClient {
    pub fn new(api_url: Url) -> Arc<Self> {
        Arc::new(Self { api_url })
    }

    fn build_url(&self, segments_in: &[&str]) -> Result<Url, ApiClientError> {
        let mut url = self.api_url.clone();
        let mut segments_out = url
            .path_segments_mut()
            .map_err(|()| ApiClientError::InvalidBaseUrl)?;
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
