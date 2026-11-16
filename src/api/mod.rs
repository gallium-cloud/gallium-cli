use crate::api::errors::ApiClientError;
use crate::helpers::auth::AccessToken;
use async_trait::async_trait;
use gallium_api_client::apis::configuration::Configuration;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_middleware::{ClientBuilder, Middleware, Next};
use std::sync::Arc;
use url::Url;

pub mod errors;

const CLI_VERSION_HEADER: &str = "Gallium-CLI";
const CLI_MESSAGE_HEADER: &str = "X-Gallium-Cli-Msg";
const USER_AGENT: &str = "Gallium Cloud CLI (https://gallium.cloud)";

pub struct ApiClient {
    config: Configuration,
}

impl ApiClient {
    pub fn new(api_url: &Url) -> Result<Arc<Self>, ApiClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            CLI_VERSION_HEADER,
            HeaderValue::from_static(clap::crate_version!()),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Arc::new(Self {
            config: Configuration {
                base_path: api_url.as_str().trim_end_matches('/').to_string(),
                user_agent: Some(USER_AGENT.to_string()),
                client: ClientBuilder::new(client).with(PrintCliMessage).build(),
                ..Configuration::default()
            },
        }))
    }

    pub fn with_access_token(&self, access_token: AccessToken) -> Arc<Self> {
        Arc::new(Self {
            config: Configuration {
                bearer_access_token: Some(access_token.0),
                ..self.config.clone()
            },
        })
    }

    pub fn config(&self) -> &Configuration {
        &self.config
    }
}

/// Paranoid validation of path params
pub fn path_param(value: &str) -> Result<&str, ApiClientError> {
    if !value.is_empty()
        && value.len() <= 64
        && value.bytes().all(|b| {
            b.is_ascii_alphabetic() || b.is_ascii_digit() || b == b'-' || b == b'_' || b == b'.'
        })
        && !value.starts_with('.')
    {
        Ok(value)
    } else {
        Err(ApiClientError::InvalidPathSegmentParameter {
            val: value.to_string(),
        })
    }
}

struct PrintCliMessage;

#[async_trait]
impl Middleware for PrintCliMessage {
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let response = next.run(req, extensions).await?;
        if let Some(msg) = response
            .headers()
            .get(CLI_MESSAGE_HEADER)
            .and_then(|h| h.to_str().ok())
        {
            eprintln!("{msg}");
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_param_accept_gallium_id_formats() {
        for s in [
            "b6b3dece-577c-4591-a406-cf089d67bf44",
            "z-9999999999999999",
            "k-ygmaoxqa4eynoypz",
            "MnuyS2DvRZy4AE7qX7LJtgBa",
            "Ma8qKdIUS-uRS4rWwLEXwADT",
            "_xtDWkOJQxKZzYXhel5a-ADO",
        ] {
            assert_eq!(path_param(s).unwrap(), s, "accepts {s}");
        }
    }

    #[test]
    fn path_param_rejects_invalid_values() {
        for s in [
            "/abc", "ab/c", "abc/", " abc", "abc ", "abc\n", "ab\0c", ".", "..", ".hidden",
        ] {
            assert!(path_param(s).is_err(), "rejects {s}");
        }
    }
}
