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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .get_schedules_v1workflows_schedules_get(
    ///             &GetSchedulesV1WorkflowsSchedulesGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_schedules_v1workflows_schedules_get(
        &self,
        request: &GetSchedulesV1WorkflowsSchedulesGetQueryRequest,
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .schedule_workflow_v1workflows_schedules_post(
    ///             &WorkflowScheduleRequest {
    ///                 schedule: ScheduleDefinition {
    ///                     input: serde_json::json!({"key":"value"}),
    ///                     ..Default::default()
    ///                 },
    ///                 deployment_name: None,
    ///                 schedule_id: None,
    ///                 workflow_identifier: None,
    ///                 workflow_registration_id: None,
    ///                 workflow_task_queue: None,
    ///                 workflow_version_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn schedule_workflow_v1workflows_schedules_post(
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .get_schedule_v1workflows_schedules_schedule_id_get(&"schedule_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_schedule_v1workflows_schedules_schedule_id_get(
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .unschedule_workflow_v1workflows_schedules_schedule_id_delete(
    ///             &"schedule_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unschedule_workflow_v1workflows_schedules_schedule_id_delete(
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .update_schedule_v1workflows_schedules_schedule_id_patch(
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
    pub async fn update_schedule_v1workflows_schedules_schedule_id_patch(
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .pause_schedule_v1workflows_schedules_schedule_id_pause_post(
    ///             &"schedule_id".to_string(),
    ///             &Some(WorkflowSchedulePauseRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pause_schedule_v1workflows_schedules_schedule_id_pause_post(
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .resume_schedule_v1workflows_schedules_schedule_id_resume_post(
    ///             &"schedule_id".to_string(),
    ///             &Some(WorkflowSchedulePauseRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resume_schedule_v1workflows_schedules_schedule_id_resume_post(
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .workflows
    ///         .schedules
    ///         .trigger_schedule_v1workflows_schedules_schedule_id_trigger_post(
    ///             &"schedule_id".to_string(),
    ///             &Some(WorkflowScheduleTriggerRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn trigger_schedule_v1workflows_schedules_schedule_id_trigger_post(
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
