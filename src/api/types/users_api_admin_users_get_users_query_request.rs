pub use crate::prelude::*;

/// Query parameters for users_api_admin_users_get_users
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiAdminUsersGetUsersQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Email address to filter users and invitations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl UsersApiAdminUsersGetUsersQueryRequest {
    pub fn builder() -> UsersApiAdminUsersGetUsersQueryRequestBuilder {
        <UsersApiAdminUsersGetUsersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiAdminUsersGetUsersQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    email: Option<String>,
}

impl UsersApiAdminUsersGetUsersQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsersApiAdminUsersGetUsersQueryRequest`].
    pub fn build(self) -> Result<UsersApiAdminUsersGetUsersQueryRequest, BuildError> {
        Ok(UsersApiAdminUsersGetUsersQueryRequest {
            page: self.page,
            page_size: self.page_size,
            email: self.email,
        })
    }
}
