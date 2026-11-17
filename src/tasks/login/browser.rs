use crate::args::GlobalArguments;
use crate::helpers::loopback::LoopbackListener;
use crate::task_common::error::TaskError;
use gallium_api_client::apis::user_api;
use gallium_api_client::models::{CliTokenRequest, GalliumLoginResponse};
use oauth2::{CsrfToken, PkceCodeChallenge};
use snafu::ResultExt;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use url::Url;

const CODE_TIMEOUT: Duration = Duration::from_secs(5 * 60);

pub async fn browser_login(args: &GlobalArguments) -> Result<GalliumLoginResponse, TaskError> {
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let state = CsrfToken::new_random();
    let listener = LoopbackListener::bind().await?;
    let hostname = gethostname::gethostname().to_string_lossy().into_owned();
    let authorize_url = build_authorize_url(
        args.get_console_url(),
        challenge.as_str(),
        &listener.redirect_uri(),
        state.secret(),
        &hostname,
    )?;
    let done_url = format!("{}/cli/done", args.get_console_url());

    if open::that_detached(authorize_url.as_str()).is_ok() {
        println!("Browser didn't open? Use the URL below to complete authentication.");
    } else {
        eprintln!("Failed to open browser - use the URL below to complete authentication.");
    }

    println!("URL: {authorize_url}");

    let code = tokio::select! {
        code = listener.wait_for_code(state.secret(), &done_url) => code?,
        code = read_code_stdin() => code?,
        _ = tokio::time::sleep(CODE_TIMEOUT) => {
            return Err(TaskError::InvalidStateForCommand {
                command: "login",
                reason: "Timed out waiting for login to complete".into(),
            });
        }
    };

    Ok(user_api::cli_token(
        args.build_unauthed_api_client()?.config(),
        CliTokenRequest {
            code,
            code_verifier: verifier.into_secret(),
        },
    )
    .await?)
}

fn build_authorize_url(
    console_url: &str,
    code_challenge: &str,
    redirect_uri: &str,
    state: &str,
    hostname: &str,
) -> Result<Url, TaskError> {
    let mut url =
        Url::parse(&format!("{console_url}/cli/authorize")).map_err(|e| TaskError::Initialize {
            name: "console URL",
            source: Box::new(e),
        })?;
    url.query_pairs_mut()
        .append_pair("code_challenge", code_challenge)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", state)
        .append_pair("hostname", hostname);
    Ok(url)
}

async fn read_code_stdin() -> Result<String, TaskError> {
    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    loop {
        match lines.next_line().await {
            Ok(Some(line)) if !line.trim().is_empty() => return Ok(line.trim().to_string()),
            Ok(Some(_)) => continue,
            Ok(None) => std::future::pending::<()>().await,
            Err(e) => return Err(e).whatever_context("read the pasted code"),
        }
    }
}
