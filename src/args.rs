use crate::api::ApiClient;
use crate::api::auth_session::AuthSession;
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

    #[arg(long, default_value = "https://console.gallium.cloud/", hide = true)]
    console_url: String,

    /// Optionally specify the org slug of the organisation containing the instance you wish to connect to.
    #[arg(short, long, default_missing_value= Option::None)]
    pub gallium_org: Option<String>,

    #[clap(long, hide = true, default_value_t = 120)]
    pub token_expiry_margin_seconds: i64,
}

#[derive(clap::Subcommand)]
pub enum Action {
    #[command(flatten)]
    Generated(crate::generated::GeneratedAction),

    #[clap(hide = true)]
    DlQemuImg,

    #[clap(hide = true)]
    Proxy(crate::tasks_internal::proxy::ProxyArguments),

    /// Export virtual disks from a Gallium deployment
    Export(crate::tasks::export::ExportArguments),

    /// Import virtual disks into a Gallium deployment
    Import(crate::tasks::import::ImportArguments),

    /// Login to your Gallium account
    Login(crate::tasks::login::LoginArguments),

    /// Clear saved login token
    Logout,

    /// SSH to an instance on a Gallium server
    Ssh(crate::tasks::ssh::SshArguments),

    /// Print version information
    Version,
}

impl GlobalArguments {
    pub fn get_api_url(&self) -> &str {
        self.api_url.strip_suffix("/").unwrap_or(&self.api_url)
    }

    /// The web console that approves browser logins.
    pub fn get_console_url(&self) -> &str {
        self.console_url
            .strip_suffix("/")
            .unwrap_or(&self.console_url)
    }

    fn parse_api_url(&self) -> Result<url::Url, TaskError> {
        self.get_api_url()
            .parse()
            .map_err(|e: url::ParseError| TaskError::Initialize {
                name: "api client",
                source: Box::new(e),
            })
    }

    pub fn build_unauthed_api_client(&self) -> Result<Arc<ApiClient>, TaskError> {
        ApiClient::new(&self.parse_api_url()?).map_err(initialize_error)
    }

    pub async fn build_authed_api_client(&self) -> Result<Arc<ApiClient>, TaskError> {
        let session = AuthSession::load(self).await?;
        ApiClient::with_session(&self.parse_api_url()?, session).map_err(initialize_error)
    }
}

fn initialize_error(e: crate::api::errors::ApiClientError) -> TaskError {
    TaskError::Initialize {
        name: "api client",
        source: Box::new(e),
    }
}
