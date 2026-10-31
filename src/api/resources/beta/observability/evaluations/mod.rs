use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod runs;
pub use runs::RunsClient;
pub mod records;
pub use records::RecordsClient2;
pub mod optimizations;
pub use optimizations::OptimizationsClient;
pub mod projects;
pub use projects::ProjectsClient;
pub struct EvaluationsClient {
    pub http_client: HttpClient,
    pub runs: RunsClient,
    pub records: RecordsClient2,
    pub optimizations: OptimizationsClient,
    pub projects: ProjectsClient,
}

impl EvaluationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            runs: RunsClient::new(config.clone())?,
            records: RecordsClient2::new(config.clone())?,
            optimizations: OptimizationsClient::new(config.clone())?,
            projects: ProjectsClient::new(config.clone())?,
        })
    }

    /// Get evaluation by slug
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .evaluations
    ///         .get_evaluation_v2v1observability_evaluations_v2evaluation_slug_get(
    ///             &"evaluation_slug".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_evaluation_v2v1observability_evaluations_v2evaluation_slug_get(
        &self,
        evaluation_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<EvaluationV2, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/evaluations_v2/{}", evaluation_slug),
                None,
                None,
                options,
            )
            .await
    }

    /// List pipelines
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .evaluations
    ///         .list_pipelines_v1observability_pipelines_get(
    ///             &ListPipelinesV1ObservabilityPipelinesGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_pipelines_v1observability_pipelines_get(
        &self,
        request: &ListPipelinesV1ObservabilityPipelinesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PipelinesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/pipelines",
                None,
                QueryBuilder::new()
                    .serialize("enabled", request.enabled.clone())
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a pipeline
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .evaluations
    ///         .create_pipeline_v1observability_pipelines_post(
    ///             &CreatePipelineRequest {
    ///                 definitions: vec![LlmJudgeReference {
    ///                     slug: "slug".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 name: "name".to_string(),
    ///                 selectors: vec![PipelineConfigSelector {
    ///                     filter: None,
    ///                     source_kind: SourceKind::Span,
    ///                 }],
    ///                 description: None,
    ///                 enabled: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_pipeline_v1observability_pipelines_post(
        &self,
        request: &CreatePipelineRequest,
        options: Option<RequestOptions>,
    ) -> Result<Pipeline, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/pipelines",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a pipeline
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .evaluations
    ///         .get_pipeline_v1observability_pipelines_pipeline_id_get(&"pipeline_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_pipeline_v1observability_pipelines_pipeline_id_get(
        &self,
        pipeline_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Pipeline, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/pipelines/{}", pipeline_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a pipeline
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .evaluations
    ///         .update_pipeline_v1observability_pipelines_pipeline_id_put(
    ///             &"pipeline_id".to_string(),
    ///             &UpdatePipelineRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_pipeline_v1observability_pipelines_pipeline_id_put(
        &self,
        pipeline_id: &str,
        request: &UpdatePipelineRequest,
        options: Option<RequestOptions>,
    ) -> Result<Pipeline, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/observability/pipelines/{}", pipeline_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a pipeline
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .evaluations
    ///         .delete_pipeline_v1observability_pipelines_pipeline_id_delete(
    ///             &"pipeline_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_pipeline_v1observability_pipelines_pipeline_id_delete(
        &self,
        pipeline_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/observability/pipelines/{}", pipeline_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List evaluations for a project
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client.beta.observability.evaluations.get_project_evaluations_v2v1observability_projects_project_slug_evaluations_v2get(&"project_slug".to_string(), &GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequest {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn get_project_evaluations_v2v1observability_projects_project_slug_evaluations_v2get(
        &self,
        project_slug: &str,
        request: &GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EvaluationsV2, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/projects/{}/evaluations_v2", project_slug),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }
}
