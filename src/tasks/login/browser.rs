use crate::args::GlobalArguments;
use crate::helpers::loopback::LoopbackListener;
use crate::task_common::error::TaskError;
use crate::tasks::login::LoginArguments;
use gallium_api_client::apis::user_api;
use gallium_api_client::models::{CliTokenRequest, GalliumLoginResponse};
use oauth2::{CsrfToken, PkceCodeChallenge};
use snafu::ResultExt;
use std::io::BufRead;
use std::time::Duration;
use url::Url;

const CODE_TIMEOUT: Duration = Duration::from_secs(5 * 60);

pub async fn browser_login(
    global_args: &GlobalArguments,
    login_args: &LoginArguments,
) -> Result<GalliumLoginResponse, TaskError> {
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let state = CsrfToken::new_random();
    let listener = LoopbackListener::bind().await?;
    let hostname = gethostname::gethostname().to_string_lossy().into_owned();
    let authorize_url = build_authorize_url(
        global_args.get_console_url(),
        challenge.as_str(),
        &listener.redirect_uri(),
        state.secret(),
        &hostname,
    )?;
    let done_url = format!("{}/cli/done", global_args.get_console_url());

    if !login_args.no_auto_open {
        if open::that_detached(authorize_url.as_str()).is_ok() {
            println!("Browser didn't open? Use the URL below to complete authentication.");
        } else {
            eprintln!("Failed to open browser - use the URL below to complete authentication.");
        }
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
        global_args.build_unauthed_api_client()?.config(),
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
    let (code_tx, code_rx) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        let _: Result<(), _> = code_tx.send(first_nonblank_line(std::io::stdin().lock()));
    });
    match code_rx.await {
        Ok(Ok(Some(code))) => Ok(code),
        Ok(Ok(None)) | Err(_) => std::future::pending().await,
        Ok(Err(e)) => Err(e).whatever_context("read the pasted code"),
    }
}

fn first_nonblank_line(input: impl BufRead) -> std::io::Result<Option<String>> {
    for line in input.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            return Ok(Some(line.trim().to_string()));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn authorize_url_carries_the_contract_parameters() {
        let url = build_authorize_url(
            "https://console.example",
            "chal",
            "http://127.0.0.1:4000/callback",
            "st ate",
            "dev-laptop",
        )
        .unwrap();
        assert_eq!(url.path(), "/cli/authorize");
        let pairs: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        assert_eq!(
            pairs,
            vec![
                ("code_challenge".to_string(), "chal".to_string()),
                (
                    "redirect_uri".to_string(),
                    "http://127.0.0.1:4000/callback".to_string()
                ),
                ("state".to_string(), "st ate".to_string()),
                ("hostname".to_string(), "dev-laptop".to_string()),
            ]
        );
    }
}
