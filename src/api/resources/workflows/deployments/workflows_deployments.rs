use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;

pub struct DeploymentsClient {
    pub http_client: HttpClient,
}

impl DeploymentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List Deployments
    ///
    /// # Arguments
    ///
    /// * `is_hardened` - Filter deployments by hardened status
    /// * `search` - Filter deployments by name or ID prefix
    /// * `order_by` - Field to sort by. When omitted, active and managed deployments are grouped first, then sorted by created_at. When set, results are sorted purely by the specified field with no grouping.
    /// * `order` - Sort direction. Applied to order_by when set, or within each activity group when omitted.
    /// * `limit` - Maximum number of deployments to return
    /// * `cursor` - Cursor from a previous response for pagination
    /// * `workspace_id` - Workspace ID to scope the request to. Defaults to the caller's context.
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
    ///         .deployments
    ///         .list(
    ///             &WorkflowsDeploymentsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WorkflowsDeploymentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeploymentListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workflows/deployments",
                None,
                QueryBuilder::new()
                    .bool("active_only", request.active_only.clone())
                    .serialize("is_hardened", request.is_hardened.clone())
                    .serialize("workflow_name", request.workflow_name.clone())
                    .serialize("search", request.search.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("order", request.order.clone())
                    .serialize("limit", request.limit.clone())
                    .serialize("cursor", request.cursor.clone())
                    .serialize("workspace_id", request.workspace_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create Deployment
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
    ///         .deployments
    ///         .create(
    ///             &CreateDeploymentRequest {
    ///                 name: "name".to_string(),
    ///                 spec: DeploymentWorkerSpecInput {
    ///                     github_url: "github_url".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 resources: None,
    ///                 hardened: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateDeploymentRequest,
        options: Option<RequestOptions>,
    ) -> Result<ManagedDeploymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/deployments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Deployment
    ///
    /// # Arguments
    ///
    /// * `workflow_name` - Scope serving status to this workflow
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
    ///         .deployments
    ///         .get(
    ///             &"name".to_string(),
    ///             &WorkflowsDeploymentsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        name: &str,
        request: &WorkflowsDeploymentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeploymentDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/deployments/{}", name),
                None,
                QueryBuilder::new()
                    .serialize("workflow_name", request.workflow_name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete Deployment
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
    ///         .deployments
    ///         .delete(&"name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        name: &str,
        options: Option<RequestOptions>,
    ) -> Result<ManagedDeploymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/workflows/deployments/{}", name),
                None,
                None,
                options,
            )
            .await
    }

    /// Update Deployment
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
    ///         .deployments
    ///         .update(
    ///             &"name".to_string(),
    ///             &UpdateDeploymentRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        name: &str,
        request: &UpdateDeploymentRequest,
        options: Option<RequestOptions>,
    ) -> Result<ManagedDeploymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/workflows/deployments/{}", name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Stop Deployment
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
    ///         .deployments
    ///         .stop(&"name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn stop(
        &self,
        name: &str,
        options: Option<RequestOptions>,
    ) -> Result<ManagedDeploymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/deployments/{}/stop", name),
                None,
                None,
                options,
            )
            .await
    }

    /// Start Deployment
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
    ///         .deployments
    ///         .start(&"name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn start(
        &self,
        name: &str,
        options: Option<RequestOptions>,
    ) -> Result<ManagedDeploymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/deployments/{}/start", name),
                None,
                None,
                options,
            )
            .await
    }

    /// Restart Deployment
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
    ///         .deployments
    ///         .restart(&"name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn restart(
        &self,
        name: &str,
        options: Option<RequestOptions>,
    ) -> Result<ManagedDeploymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/deployments/{}/restart", name),
                None,
                None,
                options,
            )
            .await
    }

    /// List Deployment Workers
    ///
    /// # Arguments
    ///
    /// * `worker_status` - Filter by worker activity. active=only active, inactive=only inactive, None=no filter
    /// * `limit` - Maximum number of workers to return
    /// * `cursor` - Cursor from a previous response's `next_cursor`. Resend `worker_status` unchanged alongside it.
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
    ///         .deployments
    ///         .list_workers(
    ///             &"name".to_string(),
    ///             &ListWorkersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_workers(
        &self,
        name: &str,
        request: &ListWorkersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeploymentWorkerListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/deployments/{}/workers", name),
                None,
                QueryBuilder::new()
                    .serialize("worker_status", request.worker_status.clone())
                    .int("limit", request.limit.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve logs for a deployment (across all of its workers).
    ///
    /// Use `after`/`before`/`order` on the first request to set the time range and sort order; for
    /// the next pages pass the `cursor` from the previous response (it remembers the range and order).
    ///
    /// # Arguments
    ///
    /// * `worker_name` - Filter logs by worker name
    /// * `workflow_name` - Filter logs by workflow name
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .deployments
    ///         .logs(
    ///             &"name".to_string(),
    ///             &WorkflowsDeploymentsLogsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn logs(
        &self,
        name: &str,
        request: &WorkflowsDeploymentsLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeploymentLogSearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/deployments/{}/logs", name),
                None,
                QueryBuilder::new()
                    .serialize("worker_name", request.worker_name.clone())
                    .serialize("workflow_name", request.workflow_name.clone())
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

    /// Stream logs for a deployment (all of its workers) via SSE.
    ///
    /// Resume cursor comes from the `Last-Event-ID` header or `last_event_id` query param (header wins)
    /// and takes precedence over `after`; omit all to tail from the deployment start.
    ///
    /// # Arguments
    ///
    /// * `worker_name` - Filter logs by worker name
    /// * `workflow_name` - Filter logs by workflow name
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .deployments
    ///         .stream_logs(
    ///             &"name".to_string(),
    ///             &WorkflowsDeploymentsStreamLogsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_logs(
        &self,
        name: &str,
        request: &WorkflowsDeploymentsStreamLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<StreamLogsDeploymentsResponse>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::GET,
                &format!("v1/workflows/deployments/{}/logs/stream", name),
                None,
                QueryBuilder::new()
                    .serialize("worker_name", request.worker_name.clone())
                    .serialize("workflow_name", request.workflow_name.clone())
                    .serialize("after", request.after.clone())
                    .serialize("last_event_id", request.last_event_id.clone())
                    .build(),
                options,
                None,
            )
            .await
    }
}
