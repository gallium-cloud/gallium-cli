use std::process::Command;

use time::OffsetDateTime;

pub fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");

    let git_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let git_commit_date = Command::new("git")
        .args(["log", "-1", "--format=%ci"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            let date_str = String::from_utf8_lossy(&o.stdout).trim().to_string();
            date_str
                .split_once(' ')
                .map(|d| d.0)
                .unwrap_or(&date_str)
                .to_string()
        })
        .unwrap_or_else(|| "unknown".to_string());

    let today = OffsetDateTime::now_utc().date();
    println!(
        "cargo:rustc-env=BUILD_DATE={:04}-{:02}-{:02}",
        today.year(),
        today.month() as u8,
        today.day()
    );
    println!("cargo:rustc-env=GIT_HASH={git_hash}");
    println!("cargo:rustc-env=GIT_COMMIT_DATE={git_commit_date}");
}
