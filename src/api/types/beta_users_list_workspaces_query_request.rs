pub use crate::prelude::*;

/// Query parameters for list_workspaces
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaUsersListWorkspacesQueryRequest {
    /// Return only workspaces belonging to this organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Number of workspaces to skip before returning results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Maximum number of workspaces to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl BetaUsersListWorkspacesQueryRequest {
    pub fn builder() -> BetaUsersListWorkspacesQueryRequestBuilder {
        <BetaUsersListWorkspacesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaUsersListWorkspacesQueryRequestBuilder {
    organization_id: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
}

impl BetaUsersListWorkspacesQueryRequestBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaUsersListWorkspacesQueryRequest`].
    pub fn build(self) -> Result<BetaUsersListWorkspacesQueryRequest, BuildError> {
        Ok(BetaUsersListWorkspacesQueryRequest {
            organization_id: self.organization_id,
            offset: self.offset,
            limit: self.limit,
        })
    }
}
