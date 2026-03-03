use crate::api::ApiClient;
use crate::api::command_v2_api::entities::{
    CommandApiV2DetailsResponse, CommandApiV2ProgressPutRequest, CommandApiV2ProgressResponse,
    GetCommandDetailsPathParams, UpdateCommandProgressPathParams,
};
use crate::api::errors::ApiClientError;
use derive_more::Constructor;
use reqwest::Method;
use std::sync::Arc;

//TODO: fix this in the code generator
#[allow(unused, clippy::upper_case_acronyms)]
pub mod entities;
#[derive(Constructor)]
pub struct CommandApi {
    api_client: Arc<ApiClient>,
}

impl CommandApi {
    pub async fn get_command_details(
        &self,
        path_params: &GetCommandDetailsPathParams,
    ) -> Result<CommandApiV2DetailsResponse, ApiClientError> {
        let response = self
            .api_client
            .request_authed(
                Method::GET,
                &["api", "v2", "command", &path_params.id, "details"],
            )?
            .send()
            .await?;

        self.api_client.deser_response(response).await
    }

    pub async fn update_command_progress(
        &self,
        path_params: &UpdateCommandProgressPathParams,
        request_body: &CommandApiV2ProgressPutRequest,
    ) -> Result<CommandApiV2ProgressResponse, ApiClientError> {
        let response = self
            .api_client
            .request_authed(
                Method::PUT,
                &["api", "v2", "command", &path_params.id, "progress"],
            )?
            .json(request_body)
            .send()
            .await?;

        self.api_client.deser_response(response).await
    }
}
