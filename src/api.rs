use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct TokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub mfa_required: bool,
    pub refresh_token: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: String,
    pub err_code: String,
}

pub async fn post_token(
    api_root_url: &String,
    params: &HashMap<&str, String>,
) -> anyhow::Result<String> {
    let response = reqwest::Client::new()
        .post(format!("{}/token", api_root_url))
        .json(&params)
        .header("Gallium-CLI", clap::crate_version!())
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!(response.text().await.unwrap());
    }

    Ok(response.json::<TokenResponse>().await?.access_token)
}

pub async fn post_login(
    api_root_url: &String,
    email: &String,
    password: &String,
    otp: &String,
) -> anyhow::Result<Result<LoginResponse, ErrorResponse>> {
    let response = reqwest::Client::new()
        .post(format!("{}/login", api_root_url))
        .json(&std::collections::HashMap::from([
            ("email", email),
            ("password", password),
            ("otp", otp),
        ]))
        .header("Gallium-CLI", clap::crate_version!())
        .send()
        .await?;

    if !response.status().is_success() {
        Ok(Err(response.json::<ErrorResponse>().await?))
    } else {
        Ok(Ok(response.json::<LoginResponse>().await?))
    }
}
