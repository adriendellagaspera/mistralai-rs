use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get Stream Events
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Server-Sent Events stream (use futures::StreamExt to iterate)
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
    ///         .events
    ///         .stream(
    ///             &EventsStreamQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream(
        &self,
        request: &EventsStreamQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<StreamEventsResponse>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::GET,
                "v1/workflows/events/stream",
                None,
                QueryBuilder::new()
                    .serialize("scope", request.scope.clone())
                    .string("activity_name", request.activity_name.clone())
                    .string("activity_id", request.activity_id.clone())
                    .string("workflow_name", request.workflow_name.clone())
                    .string("workflow_exec_id", request.workflow_exec_id.clone())
                    .string(
                        "root_workflow_exec_id",
                        request.root_workflow_exec_id.clone(),
                    )
                    .string(
                        "parent_workflow_exec_id",
                        request.parent_workflow_exec_id.clone(),
                    )
                    .string("stream", request.stream.clone())
                    .int("start_seq", request.start_seq.clone())
                    .serialize("metadata_filters", request.metadata_filters.clone())
                    .serialize("workflow_event_types", request.workflow_event_types.clone())
                    .build(),
                options,
                None,
            )
            .await
    }

    /// Get Workflow Events
    ///
    /// # Arguments
    ///
    /// * `root_workflow_exec_id` - Execution ID of the root workflow that initiated this execution chain.
    /// * `workflow_exec_id` - Execution ID of the workflow that emitted this event.
    /// * `workflow_run_id` - Run ID of the workflow that emitted this event.
    /// * `limit` - Maximum number of events to return.
    /// * `cursor` - Cursor for pagination.
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
    ///         .events
    ///         .list(
    ///             &EventsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &EventsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWorkflowEventResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workflows/events/list",
                None,
                QueryBuilder::new()
                    .serialize(
                        "root_workflow_exec_id",
                        request.root_workflow_exec_id.clone(),
                    )
                    .serialize("workflow_exec_id", request.workflow_exec_id.clone())
                    .serialize("workflow_run_id", request.workflow_run_id.clone())
                    .int("limit", request.limit.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
