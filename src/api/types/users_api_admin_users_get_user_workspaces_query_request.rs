pub use crate::prelude::*;

/// Query parameters for users_api_admin_users_get_user_workspaces
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiAdminUsersGetUserWorkspacesQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl UsersApiAdminUsersGetUserWorkspacesQueryRequest {
    pub fn builder() -> UsersApiAdminUsersGetUserWorkspacesQueryRequestBuilder {
        <UsersApiAdminUsersGetUserWorkspacesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiAdminUsersGetUserWorkspacesQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl UsersApiAdminUsersGetUserWorkspacesQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersApiAdminUsersGetUserWorkspacesQueryRequest`].
    pub fn build(self) -> Result<UsersApiAdminUsersGetUserWorkspacesQueryRequest, BuildError> {
        Ok(UsersApiAdminUsersGetUserWorkspacesQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
