use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ServiceAccountsClient {
    pub http_client: HttpClient,
}

impl ServiceAccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the service accounts in a workspace, or across the organization.
    ///
    /// Scoped to a workspace, this requires the Workspace admin (`workspace_admin`) role on
    /// it. Omitting the workspace lists the whole organization and requires the Organization
    /// admin (`organization_admin`) role instead.
    ///
    /// ``q`` narrows the list to accounts whose name or description contains it. ``order``
    /// sorts by creation time (ascending by default).
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
    ///         .service_accounts
    ///         .list(
    ///             &BetaServiceAccountsListQueryRequest {
    ///                 offset: 1,
    ///                 limit: 1,
    ///                 workspace_id: None,
    ///                 include_deleted: None,
    ///                 q: None,
    ///                 order: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &BetaServiceAccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListServiceAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/service-accounts",
                None,
                QueryBuilder::new()
                    .serialize("workspace_id", request.workspace_id.clone())
                    .bool("include_deleted", request.include_deleted.clone())
                    .serialize("q", request.q.clone())
                    .serialize("order", request.order.clone())
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a service account in a workspace. Requires the Workspace admin (`workspace_admin`) role.
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
    ///         .service_accounts
    ///         .create(
    ///             &CreateServiceAccountRequest {
    ///                 name: "name".to_string(),
    ///                 workspace_id: "workspace_id".to_string(),
    ///                 description: None,
    ///                 role_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateServiceAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ServiceAccount, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/service-accounts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the workspace roles that can be assigned to a service account.
    ///
    /// Requires the Workspace admin (`workspace_admin`) role.
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
    ///         .service_accounts
    ///         .list_assignable_roles(
    ///             &ListAssignableRolesQueryRequest {
    ///                 workspace_id: "workspace_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_assignable_roles(
        &self,
        request: &ListAssignableRolesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAssignableServiceAccountRolesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/service-accounts/assignable-roles",
                None,
                QueryBuilder::new()
                    .string("workspace_id", request.workspace_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a service account.
    ///
    /// Requires the `see_all_workspace_service_accounts` or `read_workspace_service_account` permission.
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
    ///         .service_accounts
    ///         .get(&"service_account_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        service_account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ServiceAccount, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/service-accounts/{}", service_account_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a service account.
    ///
    /// Requires the Workspace admin (`workspace_admin`) role.
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
    ///         .service_accounts
    ///         .delete(&"service_account_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        service_account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/service-accounts/{}", service_account_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a service account.
    ///
    /// Requires the Workspace admin (`workspace_admin`) role.
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
    ///         .service_accounts
    ///         .update(
    ///             &"service_account_id".to_string(),
    ///             &UpdateServiceAccountRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        service_account_id: &str,
        request: &UpdateServiceAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ServiceAccount, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/service-accounts/{}", service_account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the workspace roles assigned to a service account.
    ///
    /// Requires the Workspace admin (`workspace_admin`) role.
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
    ///         .service_accounts
    ///         .list_roles(&"service_account_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_roles(
        &self,
        service_account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListServiceAccountRolesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/service-accounts/{}/roles", service_account_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Replace the workspace roles assigned to a service account.
    ///
    /// Requires the Workspace admin (`workspace_admin`) role.
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
    ///         .service_accounts
    ///         .set_roles(
    ///             &"service_account_id".to_string(),
    ///             &SetServiceAccountRolesRequest {
    ///                 role_ids: vec!["7e69e493-caf6-4671-af60-84951754d9fc".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_roles(
        &self,
        service_account_id: &str,
        request: &SetServiceAccountRolesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListServiceAccountRolesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/service-accounts/{}/roles", service_account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
