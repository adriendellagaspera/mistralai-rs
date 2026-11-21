use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RunsClient2 {
    pub http_client: HttpClient,
}

impl RunsClient2 {
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
    /// * `exclude_workflow_tags` - Exclude runs of workflows tagged with any of the listed tags.
    /// * `include_internal` - Include runs of internal/technical workflows (e.g. parallel-execution)
    /// * `page_size` - Number of items per page
    /// * `next_page_token` - Token for the next page of results
    /// * `search_key` - Filter executions by search key as repeated entries (max 3, AND'd together). 'key:value' matches a similar value, 'key==:value' matches the exact value, a bare 'key' matches executions that have the key set at all, and 'key==' matches executions whose value for the key is null.
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
    ///         .workflows
    ///         .runs
    ///         .list_runs(
    ///             &ListRunsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_runs(
        &self,
        request: &ListRunsQueryRequest,
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
                    .serialize(
                        "exclude_workflow_tags",
                        request.exclude_workflow_tags.clone(),
                    )
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
    ///         .workflows
    ///         .runs
    ///         .get_run(&"run_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_run(
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
    ///         .workflows
    ///         .runs
    ///         .get_run_history(
    ///             &"run_id".to_string(),
    ///             &GetRunHistoryQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_run_history(
        &self,
        run_id: &str,
        request: &GetRunHistoryQueryRequest,
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
