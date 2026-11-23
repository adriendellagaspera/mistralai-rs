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

    /// List Organization members and pending invitations.
    ///
    /// # Arguments
    ///
    /// * `page` - Page number to return.
    /// * `page_size` - Maximum number of results per page.
    /// * `email` - Email address to filter users and invitations.
    /// * `include_inactive` - Also return deactivated and blocked members.
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
    ///         .list(
    ///             &BetaAdminUsersListQueryRequest {
    ///                 include_inactive: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &BetaAdminUsersListQueryRequest,
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
                    .bool("include_inactive", request.include_inactive.clone())
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
    ///         .create(
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
    pub async fn create(
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
    ///     client.beta.admin.users.get_invite(None).await;
    /// }
    /// ```
    pub async fn get_invite(
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
    ///         .invite(
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
    pub async fn invite(
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
    ///         .delete_invite(&"invite_uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_invite(
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

    /// Get details for an Organization member, including deactivated and blocked members.
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
    ///         .get(&"user_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
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

    /// Remove a member from the Organization, whatever their account status.
    ///
    /// Members whose email is on one of the Organization's verified domains cannot be removed: deactivate them instead.
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
    ///         .delete(&"user_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
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
    ///         .update(
    ///             &"user_id".to_string(),
    ///             &AdminOrganizationMemberUpdate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
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

    /// Deactivate a user whose email address is on one of the Organization's verified domains: end their sessions,
    /// delete their API keys and release their seats. Their memberships, groups and roles are kept, so that
    /// `POST /users/{user_id}/reactivate` can give their access back.
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
    ///         .deactivate(&"user_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn deactivate(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeactivateUserOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/admin/users/{}/deactivate", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Reactivate a deactivated user whose email address is on one of the Organization's verified domains: give
    /// them their access back, with their seats when free ones remain.
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
    ///         .reactivate(&"user_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn reactivate(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReactivateUserOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/admin/users/{}/reactivate", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List the Workspaces an Organization member belongs to, with their roles.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID.
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
    ///         .users
    ///         .list_workspaces(
    ///             &"user_id".to_string(),
    ///             &ListWorkspacesQueryRequest {
    ///                 page: Some(1),
    ///                 page_size: Some(25),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_workspaces(
        &self,
        user_id: &str,
        request: &ListWorkspacesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdminUserWorkspacesOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/admin/users/{}/workspaces", user_id),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }
}
