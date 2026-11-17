use crate::api::auth_session::AuthSession;
use derive_more::Constructor;
use gallium_api_client::apis::configuration::Configuration;
use http::HeaderValue;
use http::header::AUTHORIZATION;
use reqwest_middleware::{Middleware, Next};
use std::sync::Arc;

#[derive(Constructor)]
pub struct AuthMiddleware {
    session: Arc<AuthSession>,
    unauthed_config: Configuration,
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let token = self
            .session
            .access_token(&self.unauthed_config)
            .await
            .map_err(reqwest_middleware::Error::middleware)?;
        let auth_header = HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(reqwest_middleware::Error::middleware)?;
        req.headers_mut().insert(AUTHORIZATION, auth_header);
        next.run(req, extensions).await
    }
}
