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
    /// * `workspace_uuid` - Filter groups to those assigned to this Workspace. Assignments to archived Workspaces never match.
    /// * `role` - Filter groups to those holding this role on a Workspace. When combined with workspace_uuid, the role must be held on that Workspace. Assignments to archived Workspaces never match.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .list(
    ///             &BetaAdminUserGroupsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &BetaAdminUserGroupsListQueryRequest,
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
                    .serialize("workspace_uuid", request.workspace_uuid.clone())
                    .serialize("role", request.role.clone())
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .create(
    ///             &AdminUserGroupIn {
    ///                 name: "Engineering Team".to_string(),
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
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

    /// Deprecated: provision all users from a user group to a workspace with a specific role.
    ///
    /// Use `POST /user-groups/{group_uuid}/workspaces` with `role_names` to assign the group to a workspace, and
    /// `PATCH /user-groups/{group_uuid}/workspaces/{workspace_uuid}` to change the roles of an existing assignment.
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .provision_to_workspace(
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
    pub async fn provision_to_workspace(
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .get(&"group_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .delete(&"group_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
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

    /// Update User Group
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .update(
    ///             &"group_uuid".to_string(),
    ///             &AdminUpdateUserGroupIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .list_members(
    ///             &"group_uuid".to_string(),
    ///             &ListMembersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_members(
        &self,
        group_uuid: &str,
        request: &ListMembersQueryRequest,
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .add_users(
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
    pub async fn add_users(
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .remove_users(
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
    pub async fn remove_users(
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client.beta.admin.user_groups.update_organization_role(&"group_uuid".to_string(), &AdminUpdateUserGroupOrganizationRoleIn {
    ///         organization_role: AdminUpdateUserGroupOrganizationRoleInOrganizationRole::AdminUpdateUserGroupOrganizationRoleInOrganizationRoleZero(AdminUpdateUserGroupOrganizationRoleInOrganizationRoleZero::Member)
    ///     }, None).await;
    /// }
    /// ```
    pub async fn update_organization_role(
        &self,
        group_uuid: &str,
        request: &AdminUpdateUserGroupOrganizationRoleIn,
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .list_workspace_assignments(
    ///             &"group_uuid".to_string(),
    ///             &ListWorkspaceAssignmentsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_workspace_assignments(
        &self,
        group_uuid: &str,
        request: &ListWorkspaceAssignmentsQueryRequest,
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .assign_to_workspace(
    ///             &"group_uuid".to_string(),
    ///             &AdminAssignGroupToWorkspaceIn {
    ///                 workspace_uuid: "workspace_uuid".to_string(),
    ///                 role: None,
    ///                 role_names: None,
    ///                 roles: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn assign_to_workspace(
        &self,
        group_uuid: &str,
        request: &AdminAssignGroupToWorkspaceIn,
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .remove_from_workspace(
    ///             &"group_uuid".to_string(),
    ///             &"workspace_uuid".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove_from_workspace(
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
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .user_groups
    ///         .update_workspace_assignment(
    ///             &"group_uuid".to_string(),
    ///             &"workspace_uuid".to_string(),
    ///             &AdminUpdateGroupWorkspaceAssignmentIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_workspace_assignment(
        &self,
        group_uuid: &str,
        workspace_uuid: &str,
        request: &AdminUpdateGroupWorkspaceAssignmentIn,
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
}
