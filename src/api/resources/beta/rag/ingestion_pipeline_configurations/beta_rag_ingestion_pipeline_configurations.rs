use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct IngestionPipelineConfigurationsClient {
    pub http_client: HttpClient,
}

impl IngestionPipelineConfigurationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// For the current workspace, lists all of the registered ingestion pipeline configurations.
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
    ///         .beta
    ///         .rag
    ///         .ingestion_pipeline_configurations
    ///         .get_configs_v1rag_ingestion_pipeline_configurations_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_configs_v1rag_ingestion_pipeline_configurations_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<IngestionPipelineConfiguration>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/rag/ingestion_pipeline_configurations",
                None,
                None,
                options,
            )
            .await
    }

    /// Register an ingestion configuration.
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
    ///         .beta
    ///         .rag
    ///         .ingestion_pipeline_configurations
    ///         .register_config_v1rag_ingestion_pipeline_configurations_put(
    ///             &CreateIngestionPipelineConfigurationRequest {
    ///                 name: "name".to_string(),
    ///                 pipeline_composition: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn register_config_v1rag_ingestion_pipeline_configurations_put(
        &self,
        request: &CreateIngestionPipelineConfigurationRequest,
        options: Option<RequestOptions>,
    ) -> Result<IngestionPipelineConfiguration, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "v1/rag/ingestion_pipeline_configurations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update Run Info
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
    ///         .beta
    ///         .rag
    ///         .ingestion_pipeline_configurations
    ///         .update_run_info_v1rag_ingestion_pipeline_configurations_id_run_info_put(
    ///             &"id".to_string(),
    ///             &UpdateRunInfo {
    ///                 chunks_count: 1,
    ///                 execution_time: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_run_info_v1rag_ingestion_pipeline_configurations_id_run_info_put(
        &self,
        id: &str,
        request: &UpdateRunInfo,
        options: Option<RequestOptions>,
    ) -> Result<IngestionPipelineConfiguration, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/rag/ingestion_pipeline_configurations/{}/run_info", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
