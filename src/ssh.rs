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

    let access_token = match crate::login::get_access_token(&gargs.api_root_url).await {
        Ok(access_token) => access_token,
        Err(_) => {
            eprintln!(
                "Ooops, you're not logged-in. Try `{:?} login`",
                std::env::current_exe().unwrap()
            );
            return;
        }
    };

    let ws_url = match get_ws_url(&gargs.api_root_url, &access_token, &host, "22").await {
        Ok(ws_url) => ws_url,
        Err(e) => {
            eprintln!("Something went wrong: {:?}", e);
            return;
        }
    };

    let output = duct::cmd(
        "ssh",
        vec![
            String::from("-o"),
            format!(
                "ProxyCommand={} proxy \"{}\"",
                std::env::current_exe().unwrap().display(),
                ws_url
            ),
        ]
        .into_iter()
        .chain(args.additional_ssh_args.clone())
        .chain(std::iter::once(args.destination.clone())),
    )
    .unchecked()
    .run()
    .expect("ssh run");

    if let Some(code) = output.status.code() {
        std::process::exit(code);
    }
    std::process::exit(1);
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

    Ok(response.json::<WsResponse>().await?.url)
}
