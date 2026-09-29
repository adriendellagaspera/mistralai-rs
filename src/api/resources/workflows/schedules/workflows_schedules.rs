use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SchedulesClient {
    pub http_client: HttpClient,
}

impl SchedulesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get Schedules
    ///
    /// # Arguments
    ///
    /// * `workflow_name` - Filter by exact workflow name
    /// * `user_id` - Filter by user ID. Pass 'current' to resolve to the authenticated user's ID.
    /// * `status` - Filter by schedule status: 'active' or 'paused'
    /// * `search` - Prefix search query for workflow name
    /// * `page_size` - Number of items per page. Omitting this parameter fetches all results at once (deprecated — pass page_size to use pagination).
    /// * `next_page_token` - Token for the next page of results
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
    ///         .schedules
    ///         .list(
    ///             &WorkflowsSchedulesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WorkflowsSchedulesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowScheduleListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workflows/schedules",
                None,
                QueryBuilder::new()
                    .serialize("workflow_name", request.workflow_name.clone())
                    .serialize("user_id", request.user_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("search", request.search.clone())
                    .serialize("page_size", request.page_size.clone())
                    .serialize("next_page_token", request.next_page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Schedule Workflow
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
    ///         .schedules
    ///         .create(
    ///             &WorkflowScheduleRequest {
    ///                 schedule: ScheduleDefinition {
    ///                     input: serde_json::json!({"key":"value"}),
    ///                     ..Default::default()
    ///                 },
    ///                 workflow_registration_id: None,
    ///                 workflow_version_id: None,
    ///                 workflow_identifier: None,
    ///                 workflow_task_queue: None,
    ///                 schedule_id: None,
    ///                 deployment_name: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &WorkflowScheduleRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowScheduleResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workflows/schedules",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Schedule
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
    ///         .schedules
    ///         .get(&"schedule_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        schedule_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ScheduleDefinitionOutput, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workflows/schedules/{}", schedule_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Unschedule Workflow
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .delete(&"schedule_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        schedule_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/workflows/schedules/{}", schedule_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update Schedule
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
    ///         .schedules
    ///         .update(
    ///             &"schedule_id".to_string(),
    ///             &WorkflowScheduleUpdateRequest {
    ///                 schedule: PartialScheduleDefinition {
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        schedule_id: &str,
        request: &WorkflowScheduleUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkflowScheduleResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/workflows/schedules/{}", schedule_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pause Schedule
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .pause(
    ///             &"schedule_id".to_string(),
    ///             &Some(WorkflowSchedulePauseRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pause(
        &self,
        schedule_id: &str,
        request: &Option<WorkflowSchedulePauseRequest>,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/schedules/{}/pause", schedule_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Resume Schedule
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .resume(
    ///             &"schedule_id".to_string(),
    ///             &Some(WorkflowSchedulePauseRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resume(
        &self,
        schedule_id: &str,
        request: &Option<WorkflowSchedulePauseRequest>,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/schedules/{}/resume", schedule_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Trigger Schedule
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .trigger(
    ///             &"schedule_id".to_string(),
    ///             &Some(WorkflowScheduleTriggerRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn trigger(
        &self,
        schedule_id: &str,
        request: &Option<WorkflowScheduleTriggerRequest>,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workflows/schedules/{}/trigger", schedule_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
