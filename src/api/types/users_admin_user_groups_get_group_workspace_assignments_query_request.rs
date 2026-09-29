pub use crate::prelude::*;

/// Query parameters for users_admin_user_groups_get_group_workspace_assignments
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequest {
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequest {
    pub fn builder() -> UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequestBuilder {
        <UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequest`].
    pub fn build(
        self,
    ) -> Result<UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequest, BuildError> {
        Ok(
            UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequest {
                page: self.page,
                page_size: self.page_size,
            },
        )
    }
}
