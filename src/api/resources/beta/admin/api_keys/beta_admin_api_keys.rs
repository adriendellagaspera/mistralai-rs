use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient {
    pub http_client: HttpClient,
}

impl ApiKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List API keys for the Organization.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of results to return.
    /// * `offset` - Number of results to skip before returning results.
    /// * `name` - Filter API keys by name substring or the last 4 characters of the key. Matching is case-insensitive and accent-insensitive.
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
    ///         .api_keys
    ///         .users_api_admin_api_keys_get_api_keys(
    ///             &UsersAPIAdminAPIKeysGetAPIKeysQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_api_keys_get_api_keys(
        &self,
        request: &UsersApiAdminApiKeysGetApiKeysQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeysExtendedOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/api-keys",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("name", request.name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a Workspace API key.
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
    ///         .api_keys
    ///         .users_api_admin_api_keys_create_api_key(
    ///             &AdminCreateAPIKeyIn {
    ///                 workspace_uuid: "workspace_uuid".to_string(),
    ///                 user_id: "user_id".to_string(),
    ///                 name: None,
    ///                 expiration: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_api_keys_create_api_key(
        &self,
        request: &AdminCreateApiKeyIn,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeyOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/api-keys",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an API key.
    ///
    /// # Arguments
    ///
    /// * `key_id` - API key ID.
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
    ///         .api_keys
    ///         .users_api_admin_api_keys_delete_api_key(&"key_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_api_keys_delete_api_key(
        &self,
        key_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteApiKeyOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/admin/api-keys/{}", key_id),
                None,
                None,
                options,
            )
            .await
    }
}
