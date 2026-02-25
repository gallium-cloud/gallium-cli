use crate::api::ApiClient;
use crate::task_common::error::TaskError;
use std::sync::Arc;

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
    #[arg(long, default_value = "https://api.gallium.cloud/", hide = true)]
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
        self.api_url.strip_suffix("/").unwrap_or(&self.api_url)
    }

    pub fn build_api_client(&self) -> Result<Arc<ApiClient>, TaskError> {
        Ok(ApiClient::new(self.get_api_url().try_into().map_err(
            |e: url::ParseError| TaskError::Initialize {
                name: "api client",
                source: Box::new(e),
            },
        )?))
    }
}
