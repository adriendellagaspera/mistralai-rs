pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsDeploymentsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_only: Option<bool>,
    /// Filter deployments by hardened status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hardened: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    /// Filter deployments by name or ID prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Field to sort by. When omitted, active and managed deployments are grouped first, then sorted by created_at. When set, results are sorted purely by the specified field with no grouping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ListDeploymentsRequestOrderBy>,
    /// Sort direction. Applied to order_by when set, or within each activity group when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListDeploymentsRequestOrder>,
    /// Maximum number of deployments to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor from a previous response for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Workspace ID to scope the request to. Defaults to the caller's context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl WorkflowsDeploymentsListQueryRequest {
    pub fn builder() -> WorkflowsDeploymentsListQueryRequestBuilder {
        <WorkflowsDeploymentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsDeploymentsListQueryRequestBuilder {
    active_only: Option<bool>,
    is_hardened: Option<bool>,
    workflow_name: Option<String>,
    search: Option<String>,
    order_by: Option<ListDeploymentsRequestOrderBy>,
    order: Option<ListDeploymentsRequestOrder>,
    limit: Option<i64>,
    cursor: Option<String>,
    workspace_id: Option<String>,
}

impl WorkflowsDeploymentsListQueryRequestBuilder {
    pub fn active_only(mut self, value: bool) -> Self {
        self.active_only = Some(value);
        self
    }

    pub fn is_hardened(mut self, value: bool) -> Self {
        self.is_hardened = Some(value);
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn order_by(mut self, value: ListDeploymentsRequestOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    pub fn order(mut self, value: ListDeploymentsRequestOrder) -> Self {
        self.order = Some(value);
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

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsDeploymentsListQueryRequest`].
    pub fn build(self) -> Result<WorkflowsDeploymentsListQueryRequest, BuildError> {
        Ok(WorkflowsDeploymentsListQueryRequest {
            active_only: self.active_only,
            is_hardened: self.is_hardened,
            workflow_name: self.workflow_name,
            search: self.search,
            order_by: self.order_by,
            order: self.order,
            limit: self.limit,
            cursor: self.cursor,
            workspace_id: self.workspace_id,
        })
    }
}
