#[allow(unused)]
pub mod entities;

use crate::api::common_api::entities::GalliumApiErrorResponse;
use crate::api::login_api::entities::{GalliumLoginRequest, GalliumLoginResponse};
use anyhow::anyhow;
use std::collections::HashMap;

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

    response
        .json::<GalliumLoginResponse>()
        .await?
        .access_token
        .ok_or_else(|| anyhow!("null access_token"))
}

pub async fn post_login(
    api_root_url: &String,
    login_request: &GalliumLoginRequest,
) -> anyhow::Result<Result<GalliumLoginResponse, GalliumApiErrorResponse>> {
    let response = reqwest::Client::new()
        .post(format!("{}/login", api_root_url))
        .json(&login_request)
        .header("Gallium-CLI", clap::crate_version!())
        .send()
        .await?;

    if !response.status().is_success() {
        Ok(Err(response.json::<GalliumApiErrorResponse>().await?))
    } else {
        Ok(Ok(response.json::<GalliumLoginResponse>().await?))
    }
}
