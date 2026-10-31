use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct OptimizationsClient {
    pub http_client: HttpClient,
}

impl OptimizationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List optimizations with optional filters
    ///
    /// # Arguments
    ///
    /// * `project` - Filter by project slug
    /// * `evaluation` - Filter by evaluation slug
    /// * `status` - Filter by lifecycle status
    /// * `algorithm` - Filter by algorithm
    /// * `tags` - Filter by tags
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
    ///         .optimizations
    ///         .list_optimizations_v1observability_optimizations_get(
    ///             &ListOptimizationsV1ObservabilityOptimizationsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_optimizations_v1observability_optimizations_get(
        &self,
        request: &ListOptimizationsV1ObservabilityOptimizationsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOptimizationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/optimizations",
                None,
                QueryBuilder::new()
                    .serialize("project", request.project.clone())
                    .serialize("evaluation", request.evaluation.clone())
                    .serialize("status", request.status.clone())
                    .serialize("algorithm", request.algorithm.clone())
                    .serialize("tags", request.tags.clone())
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get an optimization by id (with its anchored evaluation + project)
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
    ///         .optimizations
    ///         .get_optimization_v1observability_optimizations_optimization_id_get(
    ///             &"optimization_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_optimization_v1observability_optimizations_optimization_id_get(
        &self,
        optimization_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<OptimizationDetail, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/optimizations/{}", optimization_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List an optimization's trials with their observation runs (paginated)
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
    ///     client.beta.observability.evaluations.optimizations.list_optimization_trials_v1observability_optimizations_optimization_id_trials_get(&"optimization_id".to_string(), &ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIDTrialsGetQueryRequest {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn list_optimization_trials_v1observability_optimizations_optimization_id_trials_get(
        &self,
        optimization_id: &str,
        request: &ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOptimizationTrialsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/optimizations/{}/trials", optimization_id),
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
