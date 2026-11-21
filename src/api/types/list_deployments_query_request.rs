pub use crate::prelude::*;

/// Query parameters for list_deployments
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDeploymentsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_only: Option<bool>,
    /// Filter deployments by hardened status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hardened: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    /// Filter deployments by creator's user id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Filter deployments by owner's user id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Filter deployments with at least one worker on any of these location types (OR)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_types: Option<Vec<LocationType>>,
    /// Filter deployments by name or ID prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Field to sort by. When omitted, active and managed deployments are grouped first, then sorted by created_at. When set, results are sorted purely by the specified field with no grouping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<ListDeploymentsDeploymentsRequestOrderBy>,
    /// Sort direction. Applied to order_by when set, or within each activity group when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListDeploymentsDeploymentsRequestOrder>,
    /// Maximum number of deployments to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor from a previous response for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Workspace ID to scope the request to. Defaults to the caller's context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// Filter managed deployments by source repository URL (exact match)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    /// Filter managed deployments by Dockerfile path (exact match)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_path: Option<String>,
}

impl ListDeploymentsQueryRequest {
    pub fn builder() -> ListDeploymentsQueryRequestBuilder {
        <ListDeploymentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDeploymentsQueryRequestBuilder {
    active_only: Option<bool>,
    is_hardened: Option<bool>,
    workflow_name: Option<String>,
    created_by: Option<String>,
    owner: Option<String>,
    location_types: Option<Vec<LocationType>>,
    search: Option<String>,
    order_by: Option<ListDeploymentsDeploymentsRequestOrderBy>,
    order: Option<ListDeploymentsDeploymentsRequestOrder>,
    limit: Option<i64>,
    cursor: Option<String>,
    workspace_id: Option<String>,
    repository_url: Option<String>,
    dockerfile_path: Option<String>,
}

impl ListDeploymentsQueryRequestBuilder {
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

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn location_types(mut self, value: Vec<LocationType>) -> Self {
        self.location_types = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn order_by(mut self, value: ListDeploymentsDeploymentsRequestOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    pub fn order(mut self, value: ListDeploymentsDeploymentsRequestOrder) -> Self {
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

    pub fn repository_url(mut self, value: impl Into<String>) -> Self {
        self.repository_url = Some(value.into());
        self
    }

    pub fn dockerfile_path(mut self, value: impl Into<String>) -> Self {
        self.dockerfile_path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDeploymentsQueryRequest`].
    pub fn build(self) -> Result<ListDeploymentsQueryRequest, BuildError> {
        Ok(ListDeploymentsQueryRequest {
            active_only: self.active_only,
            is_hardened: self.is_hardened,
            workflow_name: self.workflow_name,
            created_by: self.created_by,
            owner: self.owner,
            location_types: self.location_types,
            search: self.search,
            order_by: self.order_by,
            order: self.order,
            limit: self.limit,
            cursor: self.cursor,
            workspace_id: self.workspace_id,
            repository_url: self.repository_url,
            dockerfile_path: self.dockerfile_path,
        })
    }
}
