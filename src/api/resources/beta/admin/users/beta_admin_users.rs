use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UsersClient2 {
    pub http_client: HttpClient,
}

impl UsersClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List Organization and Workspace roles.
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
    ///         .users
    ///         .users_api_admin_roles_get_roles(None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_roles_get_roles(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<RolesOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/admin/roles", None, None, options)
            .await
    }

    /// List Organization members and pending invitations.
    ///
    /// # Arguments
    ///
    /// * `page` - Page number to return.
    /// * `page_size` - Maximum number of results per page.
    /// * `email` - Email address to filter users and invitations.
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
    ///         .users
    ///         .users_api_admin_users_get_users(
    ///             &UsersAPIAdminUsersGetUsersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_get_users(
        &self,
        request: &UsersApiAdminUsersGetUsersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationAdminUsersOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/users",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("email", request.email.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create Organization members.
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
    ///         .users
    ///         .users_api_admin_users_create_users(
    ///             &vec![OrganizationMemberCreate {
    ///                 email: "alice.martin@example.com".to_string(),
    ///                 first_name: "Alice".to_string(),
    ///                 last_name: "Martin".to_string(),
    ///                 ..Default::default()
    ///             }],
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_create_users(
        &self,
        request: &Vec<OrganizationMemberCreate>,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationUsersCreateOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/users",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List pending Organization invitations.
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
    ///         .users
    ///         .users_api_admin_users_get_invite(None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_get_invite(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<OrganizationUserInviteOut>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/admin/users-invite", None, None, options)
            .await
    }

    /// Invite users to the Organization.
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
    ///         .users
    ///         .users_api_admin_users_invite_users(
    ///             &OrganizationInviteIn {
    ///                 email: "alice.martin@example.com, bob.smith@example.com".to_string(),
    ///                 email_language: None,
    ///                 role: None,
    ///                 role_name: None,
    ///                 role_names: None,
    ///                 roles: None,
    ///                 subscription_seat_automatic_granting: None,
    ///                 subscription_type: None,
    ///                 subscription_types: None,
    ///                 workspace_uuids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_invite_users(
        &self,
        request: &OrganizationInviteIn,
        options: Option<RequestOptions>,
    ) -> Result<OrganizationInvitesCreateOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/users-invite",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete Invite
    ///
    /// # Arguments
    ///
    /// * `invite_uuid` - Organization invitation ID.
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
    ///         .users
    ///         .users_api_admin_users_delete_invite(&"invite_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_delete_invite(
        &self,
        invite_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/admin/users-invite/{}", invite_uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Get details for an Organization member.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID.
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
    ///         .users
    ///         .users_api_admin_users_get_user(&"user_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_get_user(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdminUserOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/admin/users/{}", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove a member from the Organization.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID.
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
    ///         .users
    ///         .users_api_admin_users_delete_user(&"user_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_delete_user(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/admin/users/{}", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an Organization member's roles and product seats.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID.
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
    ///         .users
    ///         .users_api_admin_users_update_user(
    ///             &"user_id".to_string(),
    ///             &AdminOrganizationMemberUpdate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_users_update_user(
        &self,
        user_id: &str,
        request: &AdminOrganizationMemberUpdate,
        options: Option<RequestOptions>,
    ) -> Result<AdminOrganizationMemberOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/admin/users/{}", user_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
