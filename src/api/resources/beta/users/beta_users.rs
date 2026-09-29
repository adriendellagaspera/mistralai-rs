use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UsersClient {
    pub http_client: HttpClient,
}

impl UsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get Identity
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
    ///     client.beta.users.users_api_get_identity(None).await;
    /// }
    /// ```
    pub async fn users_api_get_identity(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<UserIdentity, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/users/me", None, None, options)
            .await
    }

    /// List every organization the authenticated user is a member of.
    ///
    /// Identity-only: the caller need not have selected an organization, so this
    /// reads only the user and never scopes by the active org.
    ///
    /// # Arguments
    ///
    /// * `offset` - Number of organizations to skip before returning results.
    /// * `limit` - Maximum number of organizations to return.
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
    ///         .users
    ///         .users_api_list_organizations(
    ///             &UsersAPIListOrganizationsQueryRequest {
    ///                 offset: Some(0),
    ///                 limit: Some(100),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_list_organizations(
        &self,
        request: &UsersApiListOrganizationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrganizationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/users/me/organizations",
                None,
                QueryBuilder::new()
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List every workspace the authenticated user is a member of, across all
    /// their organizations, each tagged with the organization it belongs to.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - Return only workspaces belonging to this organization.
    /// * `offset` - Number of workspaces to skip before returning results.
    /// * `limit` - Maximum number of workspaces to return.
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
    ///         .users
    ///         .users_api_list_workspaces(
    ///             &UsersAPIListWorkspacesQueryRequest {
    ///                 offset: Some(0),
    ///                 limit: Some(100),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_list_workspaces(
        &self,
        request: &UsersApiListWorkspacesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWorkspacesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/users/me/workspaces",
                None,
                QueryBuilder::new()
                    .serialize("organization_id", request.organization_id.clone())
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
