use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UserGroupsClient {
    pub http_client: HttpClient,
}

impl UserGroupsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all user groups across the organization.
    ///
    /// # Arguments
    ///
    /// * `page` - Page number to return.
    /// * `page_size` - Maximum number of results per page.
    /// * `search` - Search term used to filter user groups by name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_get_user_groups(
    ///             &UsersAPIAdminUserGroupsGetUserGroupsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_get_user_groups(
        &self,
        request: &UsersApiAdminUserGroupsGetUserGroupsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdminUserGroupsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/user-groups",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("search", request.search.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new user group.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_create_user_group(
    ///             &AdminUserGroupIn {
    ///                 name: "Engineering Team".to_string(),
    ///                 description: None,
    ///                 target_type: None,
    ///                 parent_group_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_create_user_group(
        &self,
        request: &AdminUserGroupIn,
        options: Option<RequestOptions>,
    ) -> Result<AdminUserGroupOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/user-groups",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Provision all users from a user group to a workspace with a specific role.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_provision_group_to_workspace(
    ///             &AdminProvisionGroupToWorkspaceIn {
    ///                 user_group_uuid: "user_group_uuid".to_string(),
    ///                 workspace_uuid: "workspace_uuid".to_string(),
    ///                 workspace_role: None,
    ///                 workspace_role_name: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_provision_group_to_workspace(
        &self,
        request: &AdminProvisionGroupToWorkspaceIn,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/user-groups/provision-workspace",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific user group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_get_user_group(&"group_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_get_user_group(
        &self,
        group_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdminUserGroupOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/admin/user-groups/{}", group_uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a user group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_delete_user_group(&"group_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_delete_user_group(
        &self,
        group_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/admin/user-groups/{}", group_uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a user group.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_update_user_group(
    ///             &"group_uuid".to_string(),
    ///             &AdminUpdateUserGroupIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_update_user_group(
        &self,
        group_uuid: &str,
        request: &AdminUpdateUserGroupIn,
        options: Option<RequestOptions>,
    ) -> Result<AdminUserGroupOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/admin/user-groups/{}", group_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get members of a user group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `page` - Page number to return.
    /// * `page_size` - Maximum number of results per page.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_get_user_group_members(
    ///             &"group_uuid".to_string(),
    ///             &UsersAPIAdminUserGroupsGetUserGroupMembersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_get_user_group_members(
        &self,
        group_uuid: &str,
        request: &UsersApiAdminUserGroupsGetUserGroupMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdminUserGroupMembersOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/admin/user-groups/{}/members", group_uuid),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Assign users to a user group.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_assign_users_to_group(
    ///             &"group_uuid".to_string(),
    ///             &AdminAssignUsersToGroupIn {
    ///                 user_uuids: vec!["user_uuids".to_string()],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_assign_users_to_group(
        &self,
        group_uuid: &str,
        request: &AdminAssignUsersToGroupIn,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/admin/user-groups/{}/members", group_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove users from a user group.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_remove_users_from_group(
    ///             &"group_uuid".to_string(),
    ///             &AdminAssignUsersToGroupIn {
    ///                 user_uuids: vec!["user_uuids".to_string()],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_remove_users_from_group(
        &self,
        group_uuid: &str,
        request: &AdminAssignUsersToGroupIn,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/admin/user-groups/{}/members", group_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List workspace assignments for a user group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `page` - Page number to return.
    /// * `page_size` - Maximum number of results per page.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_admin_user_groups_get_group_workspace_assignments(
    ///             &"group_uuid".to_string(),
    ///             &UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_admin_user_groups_get_group_workspace_assignments(
        &self,
        group_uuid: &str,
        request: &UsersAdminUserGroupsGetGroupWorkspaceAssignmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupWorkspaceAssignmentsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/admin/user-groups/{}/workspaces", group_uuid),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Assign a user group to a workspace.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_assign_group_to_workspace(
    ///             &"group_uuid".to_string(),
    ///             &AssignGroupToWorkspaceIn {
    ///                 workspace_uuid: "workspace_uuid".to_string(),
    ///                 role_names: None,
    ///                 roles: None,
    ///                 role: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_assign_group_to_workspace(
        &self,
        group_uuid: &str,
        request: &AssignGroupToWorkspaceIn,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/admin/user-groups/{}/workspaces", group_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a user group from a workspace.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `workspace_uuid` - Workspace ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_admin_user_groups_remove_group_from_workspace(
    ///             &"group_uuid".to_string(),
    ///             &"workspace_uuid".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_admin_user_groups_remove_group_from_workspace(
        &self,
        group_uuid: &str,
        workspace_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/admin/user-groups/{}/workspaces/{}",
                    group_uuid, workspace_uuid
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the workspace role assignment for a user group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `workspace_uuid` - Workspace ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_admin_user_groups_update_group_workspace_assignment(
    ///             &"group_uuid".to_string(),
    ///             &"workspace_uuid".to_string(),
    ///             &UpdateGroupWorkspaceAssignmentIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_admin_user_groups_update_group_workspace_assignment(
        &self,
        group_uuid: &str,
        workspace_uuid: &str,
        request: &UpdateGroupWorkspaceAssignmentIn,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/admin/user-groups/{}/workspaces/{}",
                    group_uuid, workspace_uuid
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update the organization role for a user group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_admin_user_groups_update_user_group_organization_role(
    ///             &"group_uuid".to_string(),
    ///             &UpdateUserGroupOrganizationRoleIn {
    ///                 organization_role: UpdateUserGroupOrganizationRoleInOrganizationRole::UserRole(
    ///                     UserRole::A,
    ///                 ),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_admin_user_groups_update_user_group_organization_role(
        &self,
        group_uuid: &str,
        request: &UpdateUserGroupOrganizationRoleIn,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/admin/user-groups/{}/organization-role", group_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the groups directly nested inside this group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_get_nested_groups(&"group_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_get_nested_groups(
        &self,
        group_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<NestedGroupsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/admin/user-groups/{}/nested", group_uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Replace the set of groups directly nested inside this group.
    ///
    /// # Arguments
    ///
    /// * `group_uuid` - User group ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .users_api_admin_user_groups_set_nested_groups(
    ///             &"group_uuid".to_string(),
    ///             &SetNestedGroupsIn {
    ///                 child_group_uuids: vec!["a1b2c3d4-e5f6-7890-abcd-ef1234567890".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_user_groups_set_nested_groups(
        &self,
        group_uuid: &str,
        request: &SetNestedGroupsIn,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/admin/user-groups/{}/nested", group_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
