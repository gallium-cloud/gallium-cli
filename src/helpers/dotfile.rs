use crate::task_common::error::TaskError;
use snafu::{OptionExt, ResultExt};
use std::path::PathBuf;
use tokio::fs;

fn dotfile_path() -> Result<PathBuf, TaskError> {
    let mut path_buf =
        home::home_dir().whatever_context::<_, TaskError>("resolve home directory")?;
    path_buf.push(".gallium-cli.json");
    Ok(path_buf)
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Dotfile {
    pub refresh_tokens: std::collections::HashMap<String, String>,
}

pub async fn read_dotfile() -> Result<Dotfile, TaskError> {
    if fs::try_exists(&dotfile_path()?)
        .await
        .whatever_context::<_, TaskError>("check for existing dotfile")?
    {
        let dotfile_json = fs::read(dotfile_path()?)
            .await
            .whatever_context::<_, TaskError>("read dotfile")?;

        serde_json::from_slice(&dotfile_json).whatever_context("parse dotfile")
    } else {
        Ok(Dotfile::default())
    }
}

pub async fn write_dotfile(dotfile: &Dotfile) -> Result<(), TaskError> {
    let dotfile_json =
        serde_json::to_vec(dotfile).whatever_context::<_, TaskError>("serialize dotfile")?;
    fs::write(dotfile_path()?, dotfile_json)
        .await
        .whatever_context::<_, TaskError>("write dotfile")?;
    Ok(())
}
