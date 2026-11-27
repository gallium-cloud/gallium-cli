use crate::harness::api_mock::{api_request, gallium_org, login_response};
use crate::harness::{CliSandbox, Outcome};
use serde_json::json;
use wiremock::ResponseTemplate;
use wiremock::matchers::body_json;

#[tokio::test]
async fn pasted_code_logs_in() {
    let sandbox = CliSandbox::start().await;
    sandbox
        .mock_cli_token(ResponseTemplate::new(200).set_body_json(login_response(gallium_org())))
        .await;

    let (mut login, authorize_url) = sandbox.start_browser_login().await;
    login.type_line("pasted-code").await;

    assert_eq!(
        login.finish().await,
        authorize_url.logged_in_outcome("Logged in to Gallium org: Gallium Cloud Inc")
    );
    authorize_url
        .assert_code_exchanged(&sandbox, "pasted-code")
        .await;
}

#[tokio::test]
async fn subsequent_command_reuses_refresh_token() {
    let sandbox = CliSandbox::start().await;
    sandbox
        .mock_cli_token(ResponseTemplate::new(200).set_body_json(login_response(gallium_org())))
        .await;
    api_request("POST", "/api/token")
        .and(body_json(json!({"refreshToken": "refresh-1"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(login_response(gallium_org())))
        .expect(1)
        .mount(&sandbox.api)
        .await;

    let (mut login, _) = sandbox.start_browser_login().await;
    login.type_line("pasted-code").await;
    login.finish().await;

    assert_eq!(
        sandbox.run(&[]).await,
        Outcome {
            code: Some(0),
            stdout: "Logged in to Gallium org: Gallium Cloud Inc\n2 orgs available.\n".to_string(),
            stderr: String::new(),
        }
    );
}
