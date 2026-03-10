use crate::helpers::helper_cmd_error::HelperCommandError;
use crate::helpers::qemu::qemu_img_cmd_provider::QemuImgCmdProvider;
use cliclack::log;
use std::collections::HashSet;

pub async fn ensure_qemu_img() -> Result<QemuImgCmdProvider, HelperCommandError> {
    match QemuImgCmdProvider::find_bin().await {
        Ok(bin) => Ok(bin),
        Err(HelperCommandError::QemuImgNotFound) => {
            print_qemu_img_error().await?;
            Err(HelperCommandError::QemuImgNotFound)
        }
        Err(e) => Err(e),
    }
}

async fn print_qemu_img_error() -> Result<(), HelperCommandError> {
    log::error("qemu-img is required but was not found on your PATH.")?;
    if let Some(install_hint) = get_install_hint().await {
        log::error(format!("Install it with:\n\t{install_hint}"))?;
    }

    Ok(())
}

async fn get_install_hint() -> Option<&'static str> {
    if cfg!(target_os = "macos") {
        //TODO: is there a package that doesn't install all of qemu?
        // (we should consider shipping our own qemu-img binary on macOS)
        Some("brew install qemu")
    } else if cfg!(target_os = "linux") {
        let os_release = tokio::fs::read_to_string("/etc/os-release").await.ok()?;
        let id = parse_kv(&os_release, "ID").unwrap_or("".into());
        let id_likes: HashSet<String> = parse_kv(&os_release, "ID_LIKE")
            .map(|s| s.split(" ").map(|s| s.to_string()).collect())
            .unwrap_or_default();
        if &id == "debian" || &id == "ubuntu" || id_likes.contains("debian") {
            Some("apt install qemu-utils")
        } else if &id == "fedora"
            || id_likes.contains("fedora")
            || id_likes.contains("rhel")
            || id_likes.contains("centos")
        {
            Some("dnf install qemu-img")
        } else if &id == "arch" || id_likes.contains("arch") {
            Some("pacman -S qemu-img")
        } else if &id == "alpine" {
            Some("apk add qemu-img")
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_kv(contents: &str, key: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        let (k, v) = line.split_once('=')?;
        if k == key {
            Some(v.trim_matches('"').to_lowercase())
        } else {
            None
        }
    })
}
