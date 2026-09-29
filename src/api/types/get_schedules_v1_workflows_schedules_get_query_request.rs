pub use crate::prelude::*;

/// Query parameters for get_schedules_v1_workflows_schedules_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSchedulesV1WorkflowsSchedulesGetQueryRequest {
    /// Filter by exact workflow name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    /// Filter by user ID. Pass 'current' to resolve to the authenticated user's ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Filter by schedule status: 'active' or 'paused'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetSchedulesV1WorkflowsSchedulesGetSchedulesRequestStatus>,
    /// Prefix search query for workflow name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Number of items per page. Omitting this parameter fetches all results at once (deprecated — pass page_size to use pagination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Token for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl GetSchedulesV1WorkflowsSchedulesGetQueryRequest {
    pub fn builder() -> GetSchedulesV1WorkflowsSchedulesGetQueryRequestBuilder {
        <GetSchedulesV1WorkflowsSchedulesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSchedulesV1WorkflowsSchedulesGetQueryRequestBuilder {
    workflow_name: Option<String>,
    user_id: Option<String>,
    status: Option<GetSchedulesV1WorkflowsSchedulesGetSchedulesRequestStatus>,
    search: Option<String>,
    page_size: Option<i64>,
    next_page_token: Option<String>,
}

impl GetSchedulesV1WorkflowsSchedulesGetQueryRequestBuilder {
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: GetSchedulesV1WorkflowsSchedulesGetSchedulesRequestStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetSchedulesV1WorkflowsSchedulesGetQueryRequest`].
    pub fn build(self) -> Result<GetSchedulesV1WorkflowsSchedulesGetQueryRequest, BuildError> {
        Ok(GetSchedulesV1WorkflowsSchedulesGetQueryRequest {
            workflow_name: self.workflow_name,
            user_id: self.user_id,
            status: self.status,
            search: self.search,
            page_size: self.page_size,
            next_page_token: self.next_page_token,
        })
    }
}
