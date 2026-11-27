use crate::harness::CliSandbox;
use crate::harness::api_mock::{gallium_org, login_response};
use wiremock::ResponseTemplate;

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
