pub use crate::prelude::*;

/// Query parameters for get_workflow_registrations_v1_workflows_registrations_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequest {
    /// The workflow ID to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// The task queue to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_queue: Option<String>,
    /// Whether to only return active workflows versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_only: Option<bool>,
    /// Whether to include shared workflow versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
    /// The workflow name to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_search: Option<String>,
    /// Filter by archived state. False=exclude archived, True=only archived, None=include all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Whether to include the workflow definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_workflow: Option<bool>,
    /// Whether to only return workflows available in chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_in_chat_assistant: Option<bool>,
    /// The maximum number of workflows versions to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequest {
    pub fn builder() -> GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequestBuilder {
        <GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequestBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequestBuilder {
    workflow_id: Option<String>,
    task_queue: Option<String>,
    active_only: Option<bool>,
    include_shared: Option<bool>,
    workflow_search: Option<String>,
    archived: Option<bool>,
    with_workflow: Option<bool>,
    available_in_chat_assistant: Option<bool>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequestBuilder {
    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    pub fn task_queue(mut self, value: impl Into<String>) -> Self {
        self.task_queue = Some(value.into());
        self
    }

    pub fn active_only(mut self, value: bool) -> Self {
        self.active_only = Some(value);
        self
    }

    pub fn include_shared(mut self, value: bool) -> Self {
        self.include_shared = Some(value);
        self
    }

    pub fn workflow_search(mut self, value: impl Into<String>) -> Self {
        self.workflow_search = Some(value.into());
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn with_workflow(mut self, value: bool) -> Self {
        self.with_workflow = Some(value);
        self
    }

    pub fn available_in_chat_assistant(mut self, value: bool) -> Self {
        self.available_in_chat_assistant = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequest, BuildError> {
        Ok(
            GetWorkflowRegistrationsV1WorkflowsRegistrationsGetQueryRequest {
                workflow_id: self.workflow_id,
                task_queue: self.task_queue,
                active_only: self.active_only,
                include_shared: self.include_shared,
                workflow_search: self.workflow_search,
                archived: self.archived,
                with_workflow: self.with_workflow,
                available_in_chat_assistant: self.available_in_chat_assistant,
                limit: self.limit,
                cursor: self.cursor,
            },
        )
    }
}
