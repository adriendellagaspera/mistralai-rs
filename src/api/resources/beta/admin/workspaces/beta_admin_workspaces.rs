use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WorkspacesClient {
    pub http_client: HttpClient,
}

impl WorkspacesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List Workspaces in the Organization.
    ///
    /// # Arguments
    ///
    /// * `is_archived` - Whether to include archived Workspaces.
    /// * `page` - Page number to return.
    /// * `page_size` - Maximum number of results per page.
    /// * `search` - Search term to filter Workspaces by name.
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
    ///         .workspaces
    ///         .list(
    ///             &BetaAdminWorkspacesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &BetaAdminWorkspacesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspacesOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/workspaces",
                None,
                QueryBuilder::new()
                    .bool("is_archived", request.is_archived.clone())
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("search", request.search.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a Workspace.
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
    ///         .workspaces
    ///         .create(
    ///             &AdminWorkspaceIn {
    ///                 admin_user_id: "admin_user_id".to_string(),
    ///                 name: "Product Team".to_string(),
    ///                 add_all_org_members: None,
    ///                 description: None,
    ///                 icon: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &AdminWorkspaceIn,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceEnrichedOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/workspaces",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Archive a Workspace.
    ///
    /// # Arguments
    ///
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
    ///         .workspaces
    ///         .delete(&"workspace_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        workspace_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/admin/workspaces/{}", workspace_uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a Workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_uuid` - Workspace ID.
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
    ///         .workspaces
    ///         .update(
    ///             &"workspace_uuid".to_string(),
    ///             &UpdateWorkspaceIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        workspace_uuid: &str,
        request: &UpdateWorkspaceIn,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/admin/workspaces/{}", workspace_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Add members to a Workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_uuid` - Workspace ID.
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
    ///         .workspaces
    ///         .add_users(
    ///             &"workspace_uuid".to_string(),
    ///             &AdminWorkspaceMemberIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_users(
        &self,
        workspace_uuid: &str,
        request: &AdminWorkspaceMemberIn,
        options: Option<RequestOptions>,
    ) -> Result<AddUsersToWorkspaceOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/admin/workspaces/{}/add-users", workspace_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove members from a Workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_uuid` - Workspace ID.
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
    ///         .workspaces
    ///         .remove_users(
    ///             &"workspace_uuid".to_string(),
    ///             &RemoveWorkspaceMembersIn {
    ///                 members: vec![BaseWorkspaceMemberIn {
    ///                     user_uuid: "user_uuid".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove_users(
        &self,
        workspace_uuid: &str,
        request: &RemoveWorkspaceMembersIn,
        options: Option<RequestOptions>,
    ) -> Result<RemoveWorkspaceMembersOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/admin/workspaces/{}/remove-users", workspace_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Add or update Workspace members.
    ///
    /// # Arguments
    ///
    /// * `workspace_uuid` - Workspace ID.
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
    ///         .workspaces
    ///         .upsert_users(
    ///             &"workspace_uuid".to_string(),
    ///             &AdminWorkspaceMemberIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_users(
        &self,
        workspace_uuid: &str,
        request: &AdminWorkspaceMemberIn,
        options: Option<RequestOptions>,
    ) -> Result<AddOrUpdateUsersToWorkspaceOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/admin/workspaces/{}/users", workspace_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
