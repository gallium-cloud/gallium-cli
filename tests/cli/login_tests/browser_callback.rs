use crate::harness::api_mock::{gallium_org, login_response};
use crate::harness::{CONSOLE_URL, CliSandbox};
use wiremock::ResponseTemplate;

#[tokio::test]
async fn browser_callback_login_success() {
    let sandbox = CliSandbox::start().await;
    sandbox
        .mock_cli_token(ResponseTemplate::new(200).set_body_json(login_response(gallium_org())))
        .await;

    let (login, authorize_url) = sandbox.start_browser_login().await;
    let location = authorize_url
        .open_callback("callback-code", &authorize_url.state)
        .await;

    assert_eq!(location, format!("{CONSOLE_URL}/cli/done"));
    assert_eq!(
        login.finish().await,
        authorize_url.logged_in_outcome("Logged in to Gallium org: Gallium Cloud Inc")
    );
    authorize_url
        .assert_code_exchanged(&sandbox, "callback-code")
        .await;
}
