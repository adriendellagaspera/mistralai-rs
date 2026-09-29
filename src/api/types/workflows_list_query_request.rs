pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsListQueryRequest {
    /// Filter by workflow status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListWorkflowsRequestStatus>,
    /// Whether to include shared workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
    /// Whether to only return workflows available in chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_in_chat_assistant: Option<bool>,
    /// Filter by deployment name(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<Vec<String>>,
    /// Filter by deployment activity. active=only active, inactive=only inactive, None=no filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<ListWorkflowsRequestDeploymentStatus>,
    /// Filter by archived state. False=exclude archived, True=only archived, None=include all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Filter to workflows tagged with all listed tags (AND).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Field to sort by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ListWorkflowsRequestSortBy>,
    /// Sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListWorkflowsRequestOrder>,
    /// The cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The maximum number of workflows to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Deprecated: use deployment_status instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_only: Option<bool>,
    /// Fuzzy search query for workflow name, display name, description, or ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl WorkflowsListQueryRequest {
    pub fn builder() -> WorkflowsListQueryRequestBuilder {
        <WorkflowsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsListQueryRequestBuilder {
    status: Option<ListWorkflowsRequestStatus>,
    include_shared: Option<bool>,
    available_in_chat_assistant: Option<bool>,
    deployment_name: Option<Vec<String>>,
    deployment_status: Option<ListWorkflowsRequestDeploymentStatus>,
    archived: Option<bool>,
    tags: Option<Vec<String>>,
    sort_by: Option<ListWorkflowsRequestSortBy>,
    order: Option<ListWorkflowsRequestOrder>,
    cursor: Option<String>,
    limit: Option<i64>,
    active_only: Option<bool>,
    search: Option<String>,
}

impl WorkflowsListQueryRequestBuilder {
    pub fn status(mut self, value: ListWorkflowsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn include_shared(mut self, value: bool) -> Self {
        self.include_shared = Some(value);
        self
    }

    pub fn available_in_chat_assistant(mut self, value: bool) -> Self {
        self.available_in_chat_assistant = Some(value);
        self
    }

    pub fn deployment_name(mut self, value: Vec<String>) -> Self {
        self.deployment_name = Some(value);
        self
    }

    pub fn deployment_status(mut self, value: ListWorkflowsRequestDeploymentStatus) -> Self {
        self.deployment_status = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn sort_by(mut self, value: ListWorkflowsRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn order(mut self, value: ListWorkflowsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn active_only(mut self, value: bool) -> Self {
        self.active_only = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsListQueryRequest`].
    pub fn build(self) -> Result<WorkflowsListQueryRequest, BuildError> {
        Ok(WorkflowsListQueryRequest {
            status: self.status,
            include_shared: self.include_shared,
            available_in_chat_assistant: self.available_in_chat_assistant,
            deployment_name: self.deployment_name,
            deployment_status: self.deployment_status,
            archived: self.archived,
            tags: self.tags,
            sort_by: self.sort_by,
            order: self.order,
            cursor: self.cursor,
            limit: self.limit,
            active_only: self.active_only,
            search: self.search,
        })
    }
}
