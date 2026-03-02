mod disk_pool;
mod source_scan;
#[derive(clap::Parser)]
pub(crate) struct ImportArguments {
    #[arg(short, long)]
    /// The ID or name of the disk pool where the import should be stored
    pool: Option<String>,

    #[arg(short, long)]
    /// The deployment ID to import to
    target: String,
}
