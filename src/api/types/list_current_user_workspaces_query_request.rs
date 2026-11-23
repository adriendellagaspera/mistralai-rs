pub use crate::prelude::*;

/// Query parameters for list_current_user_workspaces
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCurrentUserWorkspacesQueryRequest {
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

impl ListCurrentUserWorkspacesQueryRequest {
    pub fn builder() -> ListCurrentUserWorkspacesQueryRequestBuilder {
        <ListCurrentUserWorkspacesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCurrentUserWorkspacesQueryRequestBuilder {
    organization_id: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
}

impl ListCurrentUserWorkspacesQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListCurrentUserWorkspacesQueryRequest`].
    pub fn build(self) -> Result<ListCurrentUserWorkspacesQueryRequest, BuildError> {
        Ok(ListCurrentUserWorkspacesQueryRequest {
            organization_id: self.organization_id,
            offset: self.offset,
            limit: self.limit,
        })
    }
}
