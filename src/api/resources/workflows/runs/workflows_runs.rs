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

    /// List Runs
    ///
    /// # Arguments
    ///
    /// * `workflow_identifier` - Filter by workflow name or id
    /// * `root_execution_id` - Filter by root execution id; returns the whole execution tree (the root and all its descendant sub-workflows).
    /// * `search` - Search by workflow name, display name, or ID
    /// * `status` - Filter by workflow status
    /// * `deployment_name` - Filter by deployment name
    /// * `sort_by` - Field to sort by
    /// * `order` - Sort direction
    /// * `start_time_after` - Include runs with start_time >= value
    /// * `start_time_before` - Include runs with start_time <= value
    /// * `end_time_after` - Include runs with end_time >= value. Running executions (no end_time) are excluded; use the status filter to include them.
    /// * `end_time_before` - Include runs with end_time <= value. Running executions (no end_time) are excluded; use the status filter to include them.
    /// * `user_id` - Filter by user id. Use 'current' to filter by the authenticated user
    /// * `workflow_tags` - Filter to runs of workflows tagged with all listed tags (AND).
    /// * `include_internal` - Include runs of internal/technical workflows (e.g. parallel-execution)
    /// * `page_size` - Number of items per page
    /// * `next_page_token` - Token for the next page of results
    /// * `search_key` - Filter executions by search key as repeated 'key:value' entries. Each entry matches an exact key and a similar value; multiple entries are AND'd together (max 3).
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .runs
    ///         .list(
    ///             &WorkflowsRunsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WorkflowsRunsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowExecutionListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workflows/runs",
                None,
                QueryBuilder::new()
                    .serialize("workflow_identifier", request.workflow_identifier.clone())
                    .serialize("root_execution_id", request.root_execution_id.clone())
                    .serialize("search", request.search.clone())
                    .serialize("status", request.status.clone())
                    .serialize("deployment_name", request.deployment_name.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .serialize("order", request.order.clone())
                    .serialize("start_time_after", request.start_time_after.clone())
                    .serialize("start_time_before", request.start_time_before.clone())
                    .serialize("end_time_after", request.end_time_after.clone())
                    .serialize("end_time_before", request.end_time_before.clone())
                    .serialize("user_id", request.user_id.clone())
                    .serialize("workflow_tags", request.workflow_tags.clone())
                    .bool("include_internal", request.include_internal.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("next_page_token", request.next_page_token.clone())
                    .serialize("search_key", request.search_key.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get Run
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.workflows.runs.get(&"run_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/runs/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get Run History
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .runs
    ///         .history(
    ///             &"run_id".to_string(),
    ///             &WorkflowsRunsHistoryQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn history(
        &self,
        run_id: &str,
        request: &WorkflowsRunsHistoryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/runs/{}/history", run_id),
                None,
                QueryBuilder::new()
                    .bool("decode_payloads", request.decode_payloads.clone())
                    .build(),
                options,
            )
            .await
    }
}
