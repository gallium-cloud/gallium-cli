use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

fn dotfile_path() -> PathBuf {
    let mut buf = home::home_dir().expect("home dir");
    buf.push(".gallium-cli.json");
    buf
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Dotfile {
    pub refresh_tokens: std::collections::HashMap<String, String>,
}

pub async fn read_dotfile() -> Dotfile {
    tokio::fs::read_to_string(dotfile_path())
        .await
        .as_ref()
        .map_or_else(
            |_| Dotfile::default(),
            |contents| serde_json::from_str(contents).expect("valid json in the dotfile"),
        )
}

pub async fn write_dotfile(dotfile: &Dotfile) {
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
