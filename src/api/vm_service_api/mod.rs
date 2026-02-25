use crate::api::vm_service_api::entities::VncUrlResponse;
use anyhow::anyhow;

#[allow(unused)]
pub mod entities;

pub async fn get_ws_url(
    api_root_url: impl ToString,
    access_token: impl ToString,
    host: impl ToString,
    port: impl ToString,
) -> anyhow::Result<String> {
    let response = reqwest::Client::new()
        .get(format!("{}/ws/ws_for_vm_service", api_root_url.to_string()))
        .query(&[("host", host.to_string())])
        .query(&[("port", port.to_string())])
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token.to_string()),
        )
        .header("Gallium-CLI", clap::crate_version!())
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!(response.text().await.unwrap());
    }

    if let Some(msg) = response.headers().get("X-Gallium-Cli-Msg") {
        eprintln!(
            "{}",
            std::str::from_utf8(msg.as_bytes()).expect("utf-8 msg header")
        );
    }

    response
        .json::<VncUrlResponse>()
        .await?
        .url
        .ok_or_else(|| anyhow!("get_ws_url response missing url"))
}
