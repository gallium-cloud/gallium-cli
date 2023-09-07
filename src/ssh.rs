use std::os::unix::process::CommandExt;

#[derive(clap::Parser)]
pub(crate) struct SshArguments {
    destination: String,

    #[arg(last = true)]
    additional_ssh_args: Vec<String>,
}

pub(crate) async fn ssh(gargs: &crate::GlobalArguments, args: &SshArguments) {
    let host = match args.destination.split_once('@') {
        Some((_, host)) => host,
        None => args.destination.as_str(),
    };

    let access_token = crate::login::get_access_token(&gargs.api_root_url)
        .await
        .expect("access token");

    let ws_url = get_ws_url(
        &gargs.api_root_url,
        &access_token,
        &host,
        // TODO 2023.09.06: should this be possible to override? If so, what's the syntax?
        "22",
    )
    .await
    .expect("ws url");

    std::process::Command::new("ssh")
        .args([
            String::from("-o"),
            format!(
                "ProxyCommand={} proxy \"{}\"",
                std::env::current_exe().unwrap().display(),
                ws_url
            ),
        ])
        .args(args.additional_ssh_args.clone())
        .arg(args.destination.clone())
        .exec();
}

#[derive(serde::Deserialize, Debug)]
struct WsResponse {
    url: String,
}

async fn get_ws_url(
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
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!(response.text().await.unwrap());
    }

    Ok(response.json::<WsResponse>().await?.url)
}
