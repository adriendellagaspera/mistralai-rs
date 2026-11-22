pub use crate::prelude::*;

/// Query parameters for list_workspaces
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaAdminUsersListWorkspacesQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl BetaAdminUsersListWorkspacesQueryRequest {
    pub fn builder() -> BetaAdminUsersListWorkspacesQueryRequestBuilder {
        <BetaAdminUsersListWorkspacesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaAdminUsersListWorkspacesQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl BetaAdminUsersListWorkspacesQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaAdminUsersListWorkspacesQueryRequest`].
    pub fn build(self) -> Result<BetaAdminUsersListWorkspacesQueryRequest, BuildError> {
        Ok(BetaAdminUsersListWorkspacesQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
