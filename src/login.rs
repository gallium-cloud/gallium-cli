use tokio::io::AsyncWriteExt;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    mfa_required: bool,
    refresh_token: Option<String>,
}

fn refresh_token_dotfile_path() -> String {
    let mut buf = home::home_dir().expect("home dir");
    buf.push(".gallium-cli-refresh-token");
    buf.into_os_string()
        .into_string()
        .expect("dotfile path isn't unicode (!!!!!)")
}

async fn post_login(
    api_root_url: &String,
    email: &String,
    password: &String,
    otp: &String,
) -> anyhow::Result<LoginResponse> {
    let response = reqwest::Client::new()
        .post(format!("{}/login", api_root_url))
        .json(&std::collections::HashMap::from([
            ("email", email),
            ("password", password),
            ("otp", otp),
        ]))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!(response.text().await.unwrap());
    }

    Ok(response.json::<LoginResponse>().await?)
}

#[derive(serde::Deserialize, Debug)]
struct TokenResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
}

async fn post_token(api_root_url: &String, refresh_token: &String) -> anyhow::Result<String> {
    let response = reqwest::Client::new()
        .post(format!("{}/token", api_root_url))
        .json(&std::collections::HashMap::from([(
            "refreshToken",
            refresh_token,
        )]))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!(response.text().await.unwrap());
    }

    Ok(response.json::<TokenResponse>().await?.access_token)
}

pub(crate) async fn login(args: &crate::GlobalArguments) {
    let email: String = dialoguer::Input::new()
        .with_prompt("email")
        .interact_text()
        .expect("email address");
    let password: String = dialoguer::Password::new()
        .with_prompt("password")
        .interact()
        .expect("password");

    let mut login_response = post_login(&args.api_root_url, &email, &password, &String::from(""))
        .await
        .expect("login attempt");

    while login_response.mfa_required {
        let otp: String = dialoguer::Input::new()
            .with_prompt("one-time password from your authenticator")
            .interact_text()
            .expect("otp");
        login_response = post_login(&args.api_root_url, &email, &password, &otp)
            .await
            .expect("login attempt");
    }

    let refresh_token = login_response.refresh_token.expect("refresh token");

    tokio::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(refresh_token_dotfile_path())
        .await
        .expect("open dotfile")
        .write_all(refresh_token.as_bytes())
        .await
        .expect("write dotfile");
}

pub(crate) async fn logout() {
    tokio::fs::remove_file(refresh_token_dotfile_path())
        .await
        .expect("unlink dotfile");
}

pub(crate) async fn get_access_token(api_root_url: &String) -> anyhow::Result<String> {
    let refresh_token = tokio::fs::read_to_string(refresh_token_dotfile_path()).await?;

    post_token(api_root_url, &refresh_token).await
}
