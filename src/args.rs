#[derive(clap::Parser)]
#[command(version, arg_required_else_help = true)]
pub struct Invocation {
    #[command(flatten)]
    pub gargs: GlobalArguments,

    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(clap::Args)]
pub struct GlobalArguments {
    #[arg(long, default_value = "https://api.gallium.cloud/api", hide = true)]
    api_url: String,

    /// Optionally specify the org slug of the organisation containing the instance you wish to connect to.
    #[arg(short, long, default_missing_value= Option::None)]
    pub gallium_org: Option<String>,
}

#[derive(clap::Subcommand)]
pub enum Action {
    #[clap(hide = true)]
    Proxy(crate::tasks_internal::proxy::ProxyArguments),

    /// Login to your Gallium account
    Login,
    /// Clear saved login token
    Logout,

    /// SSH to an instance on a Gallium server
    Ssh(crate::tasks::ssh::SshArguments),
}

impl GlobalArguments {
    pub fn get_api_url(&self) -> &str {
        self.api_url.as_str()
    }
}
