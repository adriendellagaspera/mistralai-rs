use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;

pub struct ExecutionsClient {
    pub http_client: HttpClient,
}

impl ExecutionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Batch Cancel Workflow Executions
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
    ///         .workflows
    ///         .executions
    ///         .batch_cancel_workflow_executions_v1workflows_executions_cancel_post(
    ///             &BatchExecutionBody {
    ///                 execution_ids: vec!["execution_ids".to_string()],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_cancel_workflow_executions_v1workflows_executions_cancel_post(
        &self,
        request: &BatchExecutionBody,
        options: Option<RequestOptions>,
    ) -> Result<BatchExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/executions/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Batch Terminate Workflow Executions
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
    ///         .workflows
    ///         .executions
    ///         .batch_terminate_workflow_executions_v1workflows_executions_terminate_post(
    ///             &BatchExecutionBody {
    ///                 execution_ids: vec!["execution_ids".to_string()],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn batch_terminate_workflow_executions_v1workflows_executions_terminate_post(
        &self,
        request: &BatchExecutionBody,
        options: Option<RequestOptions>,
    ) -> Result<BatchExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/executions/terminate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Workflow Execution
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
    ///         .workflows
    ///         .executions
    ///         .get_workflow_execution_v1workflows_executions_execution_id_get(
    ///             &"execution_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_execution_v1workflows_executions_execution_id_get(
        &self,
        execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowExecutionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/executions/{}", execution_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancel Workflow Execution
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
    ///         .executions
    ///         .cancel_workflow_execution_v1workflows_executions_execution_id_cancel_post(
    ///             &"execution_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_workflow_execution_v1workflows_executions_execution_id_cancel_post(
        &self,
        execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/executions/{}/cancel", execution_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get Workflow Execution History
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
    ///         .workflows
    ///         .executions
    ///         .get_workflow_execution_history_v1workflows_executions_execution_id_history_get(
    ///             &"execution_id".to_string(),
    ///             &GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIDHistoryGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_execution_history_v1workflows_executions_execution_id_history_get(
        &self,
        execution_id: &str,
        request: &GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/history", execution_id),
                None,
                QueryBuilder::new()
                    .bool("decode_payloads", request.decode_payloads.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve logs for a workflow execution.
    ///
    /// Use `after`/`before`/`order` on the first request to set the time range and sort order; for
    /// the next pages pass the `cursor` from the previous response (it remembers the range and order).
    ///
    /// # Arguments
    ///
    /// * `run_id` - Filter logs by workflow run ID
    /// * `activity_id` - Filter logs by activity ID
    /// * `after` - Only return logs at or after this timestamp
    /// * `before` - Only return logs before this timestamp
    /// * `order` - First-page sort order: 'asc' (oldest first) or 'desc'. Ignored when `cursor` is set.
    /// * `cursor` - Pagination cursor from a previous response's `next_cursor`; carries the window and order
    /// * `limit` - Maximum number of logs to return
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
    ///         .executions
    ///         .get_workflow_execution_logs(
    ///             &"execution_id".to_string(),
    ///             &GetWorkflowExecutionLogsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_execution_logs(
        &self,
        execution_id: &str,
        request: &GetWorkflowExecutionLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExecutionLogSearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/logs", execution_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .serialize("activity_id", request.activity_id.clone())
                    .serialize("after", request.after.clone())
                    .serialize("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("cursor", request.cursor.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Stream logs for a workflow execution via SSE.
    ///
    /// Resume cursor comes from the `Last-Event-ID` header or `last_event_id` query param (header wins)
    /// and takes precedence over `after`; omit all to tail from the execution start.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Filter logs by workflow run ID
    /// * `activity_id` - Filter logs by activity ID
    /// * `after` - Start a fresh stream at this timestamp (ignored when resuming via last_event_id)
    /// * `last_event_id` - Resume from this cursor (a prior response's SSE id)
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .executions
    ///         .stream_workflow_execution_logs(
    ///             &"execution_id".to_string(),
    ///             &StreamWorkflowExecutionLogsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new()),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_workflow_execution_logs(
        &self,
        execution_id: &str,
        request: &StreamWorkflowExecutionLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<StreamWorkflowExecutionLogsExecutionsResponse>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/logs/stream", execution_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .serialize("activity_id", request.activity_id.clone())
                    .serialize("after", request.after.clone())
                    .serialize("last_event_id", request.last_event_id.clone())
                    .build(),
                options,
                None,
            )
            .await
    }

    /// Query Workflow Execution
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
    ///         .workflows
    ///         .executions
    ///         .query_workflow_execution_v1workflows_executions_execution_id_queries_post(
    ///             &"execution_id".to_string(),
    ///             &QueryInvocationBody {
    ///                 name: "name".to_string(),
    ///                 input: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn query_workflow_execution_v1workflows_executions_execution_id_queries_post(
        &self,
        execution_id: &str,
        request: &QueryInvocationBody,
        options: Option<RequestOptions>,
    ) -> Result<QueryWorkflowResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/executions/{}/queries", execution_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reset Workflow
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
    ///         .executions
    ///         .reset_workflow_v1workflows_executions_execution_id_reset_post(
    ///             &"execution_id".to_string(),
    ///             &ResetInvocationBody {
    ///                 event_id: 1,
    ///                 exclude_signals: None,
    ///                 exclude_updates: None,
    ///                 reason: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reset_workflow_v1workflows_executions_execution_id_reset_post(
        &self,
        execution_id: &str,
        request: &ResetInvocationBody,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/executions/{}/reset", execution_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Signal Workflow Execution
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
    ///         .workflows
    ///         .executions
    ///         .signal_workflow_execution_v1workflows_executions_execution_id_signals_post(
    ///             &"execution_id".to_string(),
    ///             &SignalInvocationBody {
    ///                 name: "name".to_string(),
    ///                 input: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn signal_workflow_execution_v1workflows_executions_execution_id_signals_post(
        &self,
        execution_id: &str,
        request: &SignalInvocationBody,
        options: Option<RequestOptions>,
    ) -> Result<SignalWorkflowResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/executions/{}/signals", execution_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Stream
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .executions
    ///         .stream_v1workflows_executions_execution_id_stream_get(
    ///             &"execution_id".to_string(),
    ///             &StreamV1WorkflowsExecutionsExecutionIDStreamGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_v1workflows_executions_execution_id_stream_get(
        &self,
        execution_id: &str,
        request: &StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<
        SseStream<StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponse>,
        ApiError,
    > {
        self.http_client
            .execute_sse_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/stream", execution_id),
                None,
                QueryBuilder::new()
                    .serialize("event_source", request.event_source.clone())
                    .serialize("last_event_id", request.last_event_id.clone())
                    .build(),
                options,
                None,
            )
            .await
    }

    /// Terminate Workflow Execution
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
    ///         .executions
    ///         .terminate_workflow_execution_v1workflows_executions_execution_id_terminate_post(
    ///             &"execution_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn terminate_workflow_execution_v1workflows_executions_execution_id_terminate_post(
        &self,
        execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/executions/{}/terminate", execution_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get Workflow Execution Trace Events
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
    ///         .workflows
    ///         .executions
    ///         .get_workflow_execution_trace_events(
    ///             &"execution_id".to_string(),
    ///             &GetWorkflowExecutionTraceEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_execution_trace_events(
        &self,
        execution_id: &str,
        request: &GetWorkflowExecutionTraceEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowExecutionTraceEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/trace/events", execution_id),
                None,
                QueryBuilder::new()
                    .bool("merge_same_id_events", request.merge_same_id_events.clone())
                    .bool(
                        "include_internal_events",
                        request.include_internal_events.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Get Workflow Execution Trace Info
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
    ///         .workflows
    ///         .executions
    ///         .get_workflow_execution_trace_info(&"execution_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_execution_trace_info(
        &self,
        execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ExecutionTraceInfoResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/trace/info", execution_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get Workflow Execution Trace Otel
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
    ///         .workflows
    ///         .executions
    ///         .get_workflow_execution_trace_otel(&"execution_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_execution_trace_otel(
        &self,
        execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowExecutionTraceOTelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/trace/otel", execution_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get Workflow Execution Trace Summary
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
    ///         .workflows
    ///         .executions
    ///         .get_workflow_execution_trace_summary(&"execution_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_execution_trace_summary(
        &self,
        execution_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowExecutionTraceSummaryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/executions/{}/trace/summary", execution_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update Workflow Execution
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
    ///         .workflows
    ///         .executions
    ///         .update_workflow_execution_v1workflows_executions_execution_id_updates_post(
    ///             &"execution_id".to_string(),
    ///             &UpdateInvocationBody {
    ///                 name: "name".to_string(),
    ///                 input: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_workflow_execution_v1workflows_executions_execution_id_updates_post(
        &self,
        execution_id: &str,
        request: &UpdateInvocationBody,
        options: Option<RequestOptions>,
    ) -> Result<UpdateWorkflowResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/executions/{}/updates", execution_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
