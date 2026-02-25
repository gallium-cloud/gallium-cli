use crate::task_common::error::TaskError;
use snafu::ResultExt;
use std::path::PathBuf;
use tokio::fs;
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

pub async fn read_dotfile() -> Result<Dotfile, TaskError> {
    if fs::try_exists(&dotfile_path())
        .await
        .whatever_context::<_, TaskError>("check for existing dotfile")?
    {
        let dotfile_json = fs::read(dotfile_path())
            .await
            .whatever_context::<_, TaskError>("read dotfile")?;

        serde_json::from_slice(&dotfile_json).whatever_context("parse dotfile")
    } else {
        Ok(Dotfile::default())
    }
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
