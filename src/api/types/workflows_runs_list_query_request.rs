pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsRunsListQueryRequest {
    /// Filter by workflow name or id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_identifier: Option<String>,
    /// Filter by root execution id; returns the whole execution tree (the root and all its descendant sub-workflows).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_execution_id: Option<String>,
    /// Search by workflow name, display name, or ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by workflow status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListRunsRequestStatus>,
    /// Filter by deployment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// Field to sort by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ListRunsRequestSortBy>,
    /// Sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListRunsRequestOrder>,
    /// Include runs with start_time >= value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_after: Option<DateTime<FixedOffset>>,
    /// Include runs with start_time <= value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_before: Option<DateTime<FixedOffset>>,
    /// Include runs with end_time >= value. Running executions (no end_time) are excluded; use the status filter to include them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_after: Option<DateTime<FixedOffset>>,
    /// Include runs with end_time <= value. Running executions (no end_time) are excluded; use the status filter to include them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_before: Option<DateTime<FixedOffset>>,
    /// Filter by user id. Use 'current' to filter by the authenticated user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Filter to runs of workflows tagged with all listed tags (AND).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_tags: Option<Vec<String>>,
    /// Include runs of internal/technical workflows (e.g. parallel-execution)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_internal: Option<bool>,
    /// Number of items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Token for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// Filter executions by search key as repeated 'key:value' entries. Each entry matches an exact key and a similar value; multiple entries are AND'd together (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_key: Option<Vec<String>>,
}

impl WorkflowsRunsListQueryRequest {
    pub fn builder() -> WorkflowsRunsListQueryRequestBuilder {
        <WorkflowsRunsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsRunsListQueryRequestBuilder {
    workflow_identifier: Option<String>,
    root_execution_id: Option<String>,
    search: Option<String>,
    status: Option<ListRunsRequestStatus>,
    deployment_name: Option<String>,
    sort_by: Option<ListRunsRequestSortBy>,
    order: Option<ListRunsRequestOrder>,
    start_time_after: Option<DateTime<FixedOffset>>,
    start_time_before: Option<DateTime<FixedOffset>>,
    end_time_after: Option<DateTime<FixedOffset>>,
    end_time_before: Option<DateTime<FixedOffset>>,
    user_id: Option<String>,
    workflow_tags: Option<Vec<String>>,
    include_internal: Option<bool>,
    page_size: Option<i64>,
    next_page_token: Option<String>,
    search_key: Option<Vec<String>>,
}

impl WorkflowsRunsListQueryRequestBuilder {
    pub fn workflow_identifier(mut self, value: impl Into<String>) -> Self {
        self.workflow_identifier = Some(value.into());
        self
    }

    pub fn root_execution_id(mut self, value: impl Into<String>) -> Self {
        self.root_execution_id = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListRunsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn deployment_name(mut self, value: impl Into<String>) -> Self {
        self.deployment_name = Some(value.into());
        self
    }

    pub fn sort_by(mut self, value: ListRunsRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn order(mut self, value: ListRunsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn start_time_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time_after = Some(value);
        self
    }

    pub fn start_time_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time_before = Some(value);
        self
    }

    pub fn end_time_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time_after = Some(value);
        self
    }

    pub fn end_time_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time_before = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workflow_tags(mut self, value: Vec<String>) -> Self {
        self.workflow_tags = Some(value);
        self
    }

    pub fn include_internal(mut self, value: bool) -> Self {
        self.include_internal = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    pub fn search_key(mut self, value: Vec<String>) -> Self {
        self.search_key = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsRunsListQueryRequest`].
    pub fn build(self) -> Result<WorkflowsRunsListQueryRequest, BuildError> {
        Ok(WorkflowsRunsListQueryRequest {
            workflow_identifier: self.workflow_identifier,
            root_execution_id: self.root_execution_id,
            search: self.search,
            status: self.status,
            deployment_name: self.deployment_name,
            sort_by: self.sort_by,
            order: self.order,
            start_time_after: self.start_time_after,
            start_time_before: self.start_time_before,
            end_time_after: self.end_time_after,
            end_time_before: self.end_time_before,
            user_id: self.user_id,
            workflow_tags: self.workflow_tags,
            include_internal: self.include_internal,
            page_size: self.page_size,
            next_page_token: self.next_page_token,
            search_key: self.search_key,
        })
    }
}
