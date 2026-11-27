use crate::harness::api_mock::api_request;
use crate::harness::{CONSOLE_URL, CliInvocation, CliSandbox, Outcome};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use url::Url;
use wiremock::ResponseTemplate;

const TOKEN_ROUTE: &str = "/api/cli/token";

pub struct AuthorizeUrl {
    pub text: String,
    pub code_challenge: String,
}

impl CliSandbox {
    pub async fn mock_cli_token(&self, response: ResponseTemplate) {
        api_request("POST", TOKEN_ROUTE)
            .respond_with(response)
            .expect(1)
            .mount(&self.api)
            .await;
    }

    pub async fn start_browser_login(&self) -> (CliInvocation, AuthorizeUrl) {
        let mut login = self.spawn(&["login", "--no-auto-open"]);
        let line = login.stdout_line().await;
        let text = line
            .strip_prefix("URL: ")
            .unwrap_or_else(|| panic!("expected the authorize URL, read: {line}"))
            .to_string();

        let url = Url::parse(&text).unwrap();
        assert_eq!(
            &url[..url::Position::AfterPath],
            format!("{CONSOLE_URL}/cli/authorize")
        );
        let mut query: HashMap<String, String> = url.query_pairs().into_owned().collect();
        let mut take = |name: &str| {
            query
                .remove(name)
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| panic!("no {name} in {text}"))
        };
        take("hostname");
        let authorize_url = AuthorizeUrl {
            code_challenge: take("code_challenge"),
            text,
        };
        (login, authorize_url)
    }
}

impl AuthorizeUrl {
    pub async fn assert_code_exchanged(&self, sandbox: &CliSandbox, code: &str) {
        let bodies = sandbox.requests_for_route(TOKEN_ROUTE).await;
        let [body] = bodies.as_slice() else {
            panic!("expected one token request, received {bodies:?}");
        };
        let verifier = body["codeVerifier"].as_str().unwrap();
        assert_eq!(body, &json!({"code": code, "codeVerifier": verifier}));
        assert_eq!(
            URL_SAFE_NO_PAD.encode(Sha256::digest(verifier)),
            self.code_challenge
        );
    }

    pub fn logged_in_outcome(&self, message: &str) -> Outcome {
        Outcome {
            code: Some(0),
            stdout: format!("URL: {}\n{message}\n", self.text),
            stderr: String::new(),
        }
    }
}
