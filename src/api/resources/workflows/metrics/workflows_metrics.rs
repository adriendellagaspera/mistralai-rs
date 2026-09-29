use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MetricsClient {
    pub http_client: HttpClient,
}

impl MetricsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get comprehensive metrics for a specific workflow.
    ///
    /// Args:
    /// workflow_name: The name of the workflow type to get metrics for
    /// start_time: Optional start time filter (ISO 8601 format)
    /// end_time: Optional end time filter (ISO 8601 format)
    ///
    /// Returns:
    /// WorkflowMetrics: Dictionary containing metrics:
    /// - execution_count: Total number of executions
    /// - success_count: Number of successful executions
    /// - error_count: Number of failed/terminated executions
    /// - average_latency_ms: Average execution duration in milliseconds
    /// - retry_rate: Proportion of workflows with retries
    /// - latency_over_time: Time-series data of execution durations
    ///
    /// Example:
    /// GET /v1/workflows/MyWorkflow/metrics
    /// GET /v1/workflows/MyWorkflow/metrics?start_time=2025-01-01T00:00:00Z
    /// GET /v1/workflows/MyWorkflow/metrics?start_time=2025-01-01T00:00:00Z&end_time=2025-12-31T23:59:59Z
    ///
    /// # Arguments
    ///
    /// * `start_time` - Filter workflows started after this time (ISO 8601)
    /// * `end_time` - Filter workflows started before this time (ISO 8601)
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
    ///         .workflows
    ///         .metrics
    ///         .get_workflow_metrics_v1workflows_workflow_name_metrics_get(
    ///             &"workflow_name".to_string(),
    ///             &GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_metrics_v1workflows_workflow_name_metrics_get(
        &self,
        workflow_name: &str,
        request: &GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowMetrics, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/{}/metrics", workflow_name),
                None,
                QueryBuilder::new()
                    .serialize("start_time", request.start_time.clone())
                    .serialize("end_time", request.end_time.clone())
                    .build(),
                options,
            )
            .await
    }
}
