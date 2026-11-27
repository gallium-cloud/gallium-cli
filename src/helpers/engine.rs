use crate::api::ApiClient;
use crate::helpers::output::OutputArgs;
use crate::task_common::error::{
    AppNameAmbiguousSnafu, AppNotFoundSnafu, MustSpecifyEngineSnafu, TaskError,
};
use gallium_api_client::apis::engines_api;
use gallium_api_client::models::ApplicationListItem;
use snafu::ensure;

pub async fn app_engine(
    api_client: &ApiClient,
    engine: Option<&str>,
    app_name: &str,
    output: &OutputArgs,
) -> Result<String, TaskError> {
    if let Some(engine) = engine {
        return Ok(engine.to_string());
    }
    ensure!(!output.is_json(), MustSpecifyEngineSnafu);

    let apps = engines_api::list_all_applications(api_client.config(), None, None, None).await?;
    let app = find_app(&apps.items, app_name)?;
    eprintln!(
        "Using app {app_name} on engine {} ({}).",
        app.engine_name, app.engine_slug
    );
    Ok(app.engine_slug.clone())
}

fn find_app<'a>(
    apps: &'a [ApplicationListItem],
    app_name: &str,
) -> Result<&'a ApplicationListItem, TaskError> {
    let matches: Vec<&ApplicationListItem> =
        apps.iter().filter(|app| app.name == app_name).collect();
    match matches.as_slice() {
        [app] => Ok(app),
        [] => AppNotFoundSnafu { name: app_name }.fail(),
        several => AppNameAmbiguousSnafu {
            name: app_name,
            engines: several
                .iter()
                .map(|app| format!("{} ({})", app.engine_name, app.engine_slug))
                .collect::<Vec<_>>()
                .join(", "),
        }
        .fail(),
    }
}
