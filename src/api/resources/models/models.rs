use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ModelsClient {
    pub http_client: HttpClient,
}

impl ModelsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Update a model name or description.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The ID of the model to update.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .models
    ///         .jobs_api_routes_fine_tuning_update_fine_tuned_model(
    ///             &"ft:open-mistral-7b:587a6b29:20240514:7e773925".to_string(),
    ///             &UpdateModelRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_fine_tuning_update_fine_tuned_model(
        &self,
        model_id: &str,
        request: &UpdateModelRequest,
        options: Option<RequestOptions>,
    ) -> Result<JobsApiRoutesFineTuningUpdateFineTunedModelModelsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/fine_tuning/models/{}", model_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Archive a fine-tuned model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The ID of the model to archive.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .models
    ///         .jobs_api_routes_fine_tuning_archive_fine_tuned_model(
    ///             &"ft:open-mistral-7b:587a6b29:20240514:7e773925".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_fine_tuning_archive_fine_tuned_model(
        &self,
        model_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ArchiveModelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/fine_tuning/models/{}/archive", model_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Un-archive a fine-tuned model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The ID of the model to unarchive.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .models
    ///         .jobs_api_routes_fine_tuning_unarchive_fine_tuned_model(
    ///             &"ft:open-mistral-7b:587a6b29:20240514:7e773925".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_fine_tuning_unarchive_fine_tuned_model(
        &self,
        model_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<UnarchiveModelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/fine_tuning/models/{}/archive", model_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List all models available to the user.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .models
    ///         .list_models_v1models_get(
    ///             &ListModelsV1ModelsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_models_v1models_get(
        &self,
        request: &ListModelsV1ModelsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ModelList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/models",
                None,
                QueryBuilder::new()
                    .serialize("provider", request.provider.clone())
                    .serialize("model", request.model.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve information about a model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The ID of the model to retrieve.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .models
    ///         .retrieve_model_v1models_model_id_get(
    ///             &"ft:open-mistral-7b:587a6b29:20240514:7e773925".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve_model_v1models_model_id_get(
        &self,
        model_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveModelV1ModelsModelIdGetModelsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/models/{}", model_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a fine-tuned model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The ID of the model to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .models
    ///         .delete_model_v1models_model_id_delete(
    ///             &"ft:open-mistral-7b:587a6b29:20240514:7e773925".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_model_v1models_model_id_delete(
        &self,
        model_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteModelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/models/{}", model_id),
                None,
                None,
                options,
            )
            .await
    }
}
