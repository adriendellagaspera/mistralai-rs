use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RunsClient {
    pub http_client: HttpClient,
}

impl RunsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search evaluation runs across all evaluations
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
    ///         .runs
    ///         .search_all_evaluation_runs_v2v1observability_evaluation_runs_v2search_post(
    ///             &SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequest {
    ///                 body: Some(EvaluationRunFilterInput {
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_all_evaluation_runs_v2v1observability_evaluation_runs_v2search_post(
        &self,
        request: &SearchAllEvaluationRunsV2V1ObservabilityEvaluationRunsV2SearchPostRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchAllEvaluationRunsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/evaluation-runs_v2/search",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a specific evaluation run by ID
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
    ///         .runs
    ///         .get_evaluation_run_v2v1observability_evaluation_runs_v2run_id_get(
    ///             &"run_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_evaluation_run_v2v1observability_evaluation_runs_v2run_id_get(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EvaluationRunV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/evaluation-runs_v2/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get statistics for a specific evaluation run
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
    ///     client.beta.observability.evaluations.runs.get_single_evaluation_run_statistics_v2v1observability_evaluation_runs_v2run_id_statistics_get(&"run_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_single_evaluation_run_statistics_v2v1observability_evaluation_runs_v2run_id_statistics_get(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RunStatisticsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/evaluation-runs_v2/{}/statistics", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Compute statistics for a specific evaluation run with optional filters
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
    ///     client.beta.observability.evaluations.runs.compute_filtered_run_statistics_v2v1observability_evaluation_runs_v2run_id_statistics_compute_post(&"run_id".to_string(), &Some(EvaluationRunRecordFilterInput {
    ///         ..Default::default()
    ///     }), None).await;
    /// }
    /// ```
    pub async fn compute_filtered_run_statistics_v2v1observability_evaluation_runs_v2run_id_statistics_compute_post(
        &self,
        run_id: &str,
        request: &Option<EvaluationRunRecordFilterInput>,
        options: Option<RequestOptions>,
    ) -> Result<RunStatisticsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/evaluation-runs_v2/{}/statistics/compute",
                    run_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Search evaluation runs with filters
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
    ///     client.beta.observability.evaluations.runs.search_evaluation_runs_v2v1observability_evaluations_v2evaluation_slug_runs_search_post(&"evaluation_slug".to_string(), &SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequest {
    ///         body: Some(EvaluationRunFilterInput {
    ///             ..Default::default()
    ///         }),
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn search_evaluation_runs_v2v1observability_evaluations_v2evaluation_slug_runs_search_post(
        &self,
        evaluation_slug: &str,
        request: &SearchEvaluationRunsV2V1ObservabilityEvaluationsV2EvaluationSlugRunsSearchPostRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEvaluationRunsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/evaluations_v2/{}/runs/search",
                    evaluation_slug
                ),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }
}
