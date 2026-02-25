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

    pub fn login_api(self: &Arc<Self>) -> LoginApi {
        LoginApi::new(self.clone())
    }

    pub fn vm_service_api(self: &Arc<Self>) -> VmServiceApi {
        VmServiceApi::new(self.clone())
    }
}
