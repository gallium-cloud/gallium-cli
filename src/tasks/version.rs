use crate::task_common::error::TaskError;

pub fn print_version_info() -> Result<(), TaskError> {
    println!("Gallium CLI Version : {}", env!("CARGO_PKG_VERSION"));
    println!("Commit date         : {}", env!("GIT_COMMIT_DATE"));
    println!("Build date          : {}", env!("BUILD_DATE"));
    println!("Git hash            : {}", env!("GIT_HASH"));

    Ok(())
}
