use crate::api;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;

pub(crate) async fn login(args: &crate::GlobalArguments) {
    let email: String = dialoguer::Input::new()
        .with_prompt("email")
        .interact_text()
        .expect("email address");
    let password: String = dialoguer::Password::new()
        .with_prompt("password")
        .interact()
        .expect("password");

    let mut login_response =
        match api::post_login(&args.api_root_url, &email, &password, &String::from("")).await {
            Ok(Ok(login_response)) => login_response,
            Ok(Err(e)) => {
                eprintln!("Error logging in: {}", e.error);
                return;
            }
            Err(e) => {
                eprintln!("Couldn't connect to API: {:?}", e);
                return;
            }
        };

    while login_response.mfa_required {
        let otp: String = dialoguer::Input::new()
            .with_prompt("one-time password from your authenticator")
            .interact_text()
            .expect("otp");
        login_response = match api::post_login(&args.api_root_url, &email, &password, &otp).await {
            Ok(Ok(login_response)) => login_response,
            Ok(Err(e)) => {
                eprintln!("Error logging in: {}", e.error);
                return;
            }
            Err(e) => {
                eprintln!("Couldn't connect to API: {:?}", e);
                return;
            }
        };
    }

    let refresh_token = login_response.refresh_token.expect("refresh token");

    let mut dotfile = read_dotfile().await;

    dotfile
        .refresh_tokens
        .insert(args.api_root_url.clone(), refresh_token);

    write_dotfile(&dotfile).await;
}

pub(crate) async fn logout(args: &crate::GlobalArguments) {
    let mut dotfile = read_dotfile().await;

    dotfile.refresh_tokens.remove(&args.api_root_url);

    write_dotfile(&dotfile).await;
}

pub(crate) async fn get_access_token(
    api_root_url: &String,
    org_param: &Option<String>,
) -> anyhow::Result<String> {
    let refresh_token = read_dotfile()
        .await
        .refresh_tokens
        .get(api_root_url)
        .ok_or(anyhow::anyhow!("no refresh token available"))?
        .clone();
    let mut params = HashMap::from([("refreshToken", refresh_token)]);
    if let Some(org) = org_param {
        params.insert("orgSlug", org.clone());
    }

    api::post_token(api_root_url, &params).await
}

fn dotfile_path() -> String {
    let mut buf = home::home_dir().expect("home dir");
    buf.push(".gallium-cli.json");
    buf.into_os_string()
        .into_string()
        .expect("dotfile path isn't unicode (!!!!!)")
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Dotfile {
    refresh_tokens: std::collections::HashMap<String, String>,
}

async fn read_dotfile() -> Dotfile {
    tokio::fs::read_to_string(dotfile_path())
        .await
        .as_ref()
        .map_or_else(
            |_| Dotfile::default(),
            |contents| serde_json::from_str(contents).expect("valid json in the dotfile"),
        )
}

async fn write_dotfile(dotfile: &Dotfile) {
    tokio::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(dotfile_path())
        .await
        .expect("open dotfile")
        .write_all(
            serde_json::to_string(dotfile)
                .expect("able to serialize dotfile to json")
                .as_bytes(),
        )
        .await
        .expect("write to dotfile")
}
