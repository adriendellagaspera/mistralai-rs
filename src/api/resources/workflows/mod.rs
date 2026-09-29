use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod deployments;
pub use deployments::DeploymentsClient;
pub mod executions;
pub use executions::ExecutionsClient;
pub mod runs;
pub use runs::RunsClient;
pub mod schedules;
pub use schedules::SchedulesClient;
pub mod metrics;
pub use metrics::MetricsClient;
pub struct WorkflowsClient {
    pub http_client: HttpClient,
    pub deployments: DeploymentsClient,
    pub executions: ExecutionsClient,
    pub runs: RunsClient,
    pub schedules: SchedulesClient,
    pub metrics: MetricsClient,
}

impl WorkflowsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            deployments: DeploymentsClient::new(config.clone())?,
            executions: ExecutionsClient::new(config.clone())?,
            runs: RunsClient::new(config.clone())?,
            schedules: SchedulesClient::new(config.clone())?,
            metrics: MetricsClient::new(config.clone())?,
        })
    }

    /// Get Workflows
    ///
    /// # Arguments
    ///
    /// * `status` - Filter by workflow status
    /// * `include_shared` - Whether to include shared workflows
    /// * `available_in_chat_assistant` - Whether to only return workflows available in chat assistant
    /// * `deployment_name` - Filter by deployment name(s)
    /// * `deployment_status` - Filter by deployment activity. active=only active, inactive=only inactive, None=no filter
    /// * `archived` - Filter by archived state. False=exclude archived, True=only archived, None=include all
    /// * `tags` - Filter to workflows tagged with all listed tags (AND).
    /// * `sort_by` - Field to sort by
    /// * `order` - Sort direction
    /// * `cursor` - The cursor for pagination
    /// * `limit` - The maximum number of workflows to return
    /// * `active_only` - Deprecated: use deployment_status instead
    /// * `search` - Fuzzy search query for workflow name, display name, description, or ID
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
    ///         .get_workflows_v1workflows_get(
    ///             &GetWorkflowsV1WorkflowsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflows_v1workflows_get(
        &self,
        request: &GetWorkflowsV1WorkflowsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workflows",
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .bool("include_shared", request.include_shared.clone())
                    .serialize(
                        "available_in_chat_assistant",
                        request.available_in_chat_assistant.clone(),
                    )
                    .serialize("deployment_name", request.deployment_name.clone())
                    .serialize("deployment_status", request.deployment_status.clone())
                    .serialize("archived", request.archived.clone())
                    .serialize("tags", request.tags.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .serialize("order", request.order.clone())
                    .serialize("cursor", request.cursor.clone())
                    .int("limit", request.limit.clone())
                    .bool("active_only", request.active_only.clone())
                    .serialize("search", request.search.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Bulk Archive Workflows
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
    ///         .bulk_archive_workflows_v1workflows_archive_put(
    ///             &WorkflowBulkArchiveRequest {
    ///                 workflow_ids: vec!["workflow_ids".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_archive_workflows_v1workflows_archive_put(
        &self,
        request: &WorkflowBulkArchiveRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowBulkArchiveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "v1/workflows/archive",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Workflow Registrations
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The workflow ID to filter by
    /// * `task_queue` - The task queue to filter by
    /// * `active_only` - Whether to only return active workflows versions
    /// * `include_shared` - Whether to include shared workflow versions
    /// * `workflow_search` - The workflow name to filter by
    /// * `archived` - Filter by archived state. False=exclude archived, True=only archived, None=include all
    /// * `with_workflow` - Whether to include the workflow definition
    /// * `available_in_chat_assistant` - Whether to only return workflows available in chat assistant
    /// * `limit` - The maximum number of workflows versions to return
    /// * `cursor` - The cursor for pagination
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
    ///         .get_workflow_registrations_v1workflows_registrations_get(
    ///             &GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_registrations_v1workflows_registrations_get(
        &self,
        request: &GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowRegistrationListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workflows/registrations",
                None,
                QueryBuilder::new()
                    .serialize("workflow_id", request.workflow_id.clone())
                    .serialize("task_queue", request.task_queue.clone())
                    .bool("active_only", request.active_only.clone())
                    .bool("include_shared", request.include_shared.clone())
                    .serialize("workflow_search", request.workflow_search.clone())
                    .serialize("archived", request.archived.clone())
                    .bool("with_workflow", request.with_workflow.clone())
                    .serialize(
                        "available_in_chat_assistant",
                        request.available_in_chat_assistant.clone(),
                    )
                    .int("limit", request.limit.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get Workflow Registration
    ///
    /// # Arguments
    ///
    /// * `with_workflow` - Whether to include the workflow definition
    /// * `include_shared` - Whether to include shared workflow versions
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
    ///         .get_workflow_registration_v1workflows_registrations_workflow_registration_id_get(
    ///             &"workflow_registration_id".to_string(),
    ///             &GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIDGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_registration_v1workflows_registrations_workflow_registration_id_get(
        &self,
        workflow_registration_id: &str,
        request: &GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowRegistrationGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/registrations/{}", workflow_registration_id),
                None,
                QueryBuilder::new()
                    .bool("with_workflow", request.with_workflow.clone())
                    .bool("include_shared", request.include_shared.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Execute Workflow Registration
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
    ///     client.workflows.execute_workflow_registration_v1workflows_registrations_workflow_registration_id_execute_post(&"workflow_registration_id".to_string(), &WorkflowExecutionRequest {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn execute_workflow_registration_v1workflows_registrations_workflow_registration_id_execute_post(&self, workflow_registration_id: &str, request: &WorkflowExecutionRequest, options: Option<RequestOptions>) -> Result<ExecuteWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdExecutePostWorkflowsResponse, ApiError>{
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/workflows/registrations/{}/execute",
                    workflow_registration_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Bulk Unarchive Workflows
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
    ///         .bulk_unarchive_workflows_v1workflows_unarchive_put(
    ///             &WorkflowBulkUnarchiveRequest {
    ///                 workflow_ids: vec!["workflow_ids".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn bulk_unarchive_workflows_v1workflows_unarchive_put(
        &self,
        request: &WorkflowBulkUnarchiveRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowBulkUnarchiveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "v1/workflows/unarchive",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Workflow
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
    ///         .get_workflow_v1workflows_workflow_identifier_get(&"workflow_identifier".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_workflow_v1workflows_workflow_identifier_get(
        &self,
        workflow_identifier: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/{}", workflow_identifier),
                None,
                None,
                options,
            )
            .await
    }

    /// Update Workflow
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
    ///         .update_workflow_v1workflows_workflow_identifier_put(
    ///             &"workflow_identifier".to_string(),
    ///             &WorkflowUpdateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_workflow_v1workflows_workflow_identifier_put(
        &self,
        workflow_identifier: &str,
        request: &WorkflowUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/workflows/{}", workflow_identifier),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Archive Workflow
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
    ///         .archive_workflow_v1workflows_workflow_identifier_archive_put(
    ///             &"workflow_identifier".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn archive_workflow_v1workflows_workflow_identifier_archive_put(
        &self,
        workflow_identifier: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowArchiveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/workflows/{}/archive", workflow_identifier),
                None,
                None,
                options,
            )
            .await
    }

    /// Execute Workflow
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
    ///         .execute_workflow_v1workflows_workflow_identifier_execute_post(
    ///             &"workflow_identifier".to_string(),
    ///             &WorkflowExecutionRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn execute_workflow_v1workflows_workflow_identifier_execute_post(
        &self,
        workflow_identifier: &str,
        request: &WorkflowExecutionRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExecuteWorkflowV1WorkflowsWorkflowIdentifierExecutePostWorkflowsResponse, ApiError>
    {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/{}/execute", workflow_identifier),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Unarchive Workflow
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
    ///         .unarchive_workflow_v1workflows_workflow_identifier_unarchive_put(
    ///             &"workflow_identifier".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unarchive_workflow_v1workflows_workflow_identifier_unarchive_put(
        &self,
        workflow_identifier: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowUnarchiveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/workflows/{}/unarchive", workflow_identifier),
                None,
                None,
                options,
            )
            .await
    }
}
