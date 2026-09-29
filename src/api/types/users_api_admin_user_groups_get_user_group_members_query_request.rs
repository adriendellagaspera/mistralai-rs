pub use crate::prelude::*;

/// Query parameters for users_api_admin_user_groups_get_user_group_members
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiAdminUserGroupsGetUserGroupMembersQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl UsersApiAdminUserGroupsGetUserGroupMembersQueryRequest {
    pub fn builder() -> UsersApiAdminUserGroupsGetUserGroupMembersQueryRequestBuilder {
        <UsersApiAdminUserGroupsGetUserGroupMembersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiAdminUserGroupsGetUserGroupMembersQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl UsersApiAdminUserGroupsGetUserGroupMembersQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersApiAdminUserGroupsGetUserGroupMembersQueryRequest`].
    pub fn build(
        self,
    ) -> Result<UsersApiAdminUserGroupsGetUserGroupMembersQueryRequest, BuildError> {
        Ok(UsersApiAdminUserGroupsGetUserGroupMembersQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
