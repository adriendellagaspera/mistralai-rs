pub use crate::prelude::*;

/// Query parameters for users_api_admin_user_groups_get_user_groups
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersApiAdminUserGroupsGetUserGroupsQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Search term used to filter user groups by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter groups to those assigned to this Workspace. Assignments to archived Workspaces never match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_uuid: Option<String>,
    /// Filter groups to those holding this role on a Workspace. When combined with workspace_uuid, the role must be held on that Workspace. Assignments to archived Workspaces never match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UsersApiAdminUserGroupsGetUserGroupsUserGroupsRequestRole>,
}

impl UsersApiAdminUserGroupsGetUserGroupsQueryRequest {
    pub fn builder() -> UsersApiAdminUserGroupsGetUserGroupsQueryRequestBuilder {
        <UsersApiAdminUserGroupsGetUserGroupsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersApiAdminUserGroupsGetUserGroupsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    search: Option<String>,
    workspace_uuid: Option<String>,
    role: Option<UsersApiAdminUserGroupsGetUserGroupsUserGroupsRequestRole>,
}

impl UsersApiAdminUserGroupsGetUserGroupsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    pub fn role(
        mut self,
        value: UsersApiAdminUserGroupsGetUserGroupsUserGroupsRequestRole,
    ) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersApiAdminUserGroupsGetUserGroupsQueryRequest`].
    pub fn build(self) -> Result<UsersApiAdminUserGroupsGetUserGroupsQueryRequest, BuildError> {
        Ok(UsersApiAdminUserGroupsGetUserGroupsQueryRequest {
            page: self.page,
            page_size: self.page_size,
            search: self.search,
            workspace_uuid: self.workspace_uuid,
            role: self.role,
        })
    }
}
