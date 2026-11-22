use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ConnectorsClient {
    pub http_client: HttpClient,
}

impl ConnectorsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all your custom connectors with keyset pagination and filters.
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
    ///         .connectors
    ///         .list(
    ///             &BetaConnectorsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &BetaConnectorsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedConnectors, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/connectors",
                None,
                QueryBuilder::new()
                    .serialize("query_filters", request.query_filters.clone())
                    .serialize("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new MCP or HTTP connector. You can customize its visibility, URL, and auth type.
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
    ///         .connectors
    ///         .create(
    ///             &CreateConnectorsRequest::HTTP {
    ///                 data: CreateHTTPConnectorRequest {
    ///                     description: "description".to_string(),
    ///                     name: "name".to_string(),
    ///                     server: "server".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateConnectorsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Connector, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/connectors",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a connector by its ID or name.
    ///
    /// # Arguments
    ///
    /// * `fetch_user_data` - Fetch the user-level data associated with the connector (e.g. connection credentials).
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
    ///         .connectors
    ///         .get(
    ///             &"connector_id_or_name".to_string(),
    ///             &BetaConnectorsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        connector_id_or_name: &str,
        request: &BetaConnectorsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Connector, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/connectors/{}#idOrName", connector_id_or_name),
                None,
                QueryBuilder::new()
                    .bool("fetch_user_data", request.fetch_user_data.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get the OAuth2 authorization URL for a connector to initiate user authentication.
    ///
    /// # Arguments
    ///
    /// * `method_type` - Auth method type to use for the authorization URL. Required when the connector supports multiple interactive auth methods; otherwise the sole method is selected automatically. Use this to pick a specific method (e.g. 'oauth2' vs 'github_app').
    /// * `github_installation_link` - Only valid with method_type=oauth2. When true, returns a GitHub App installation URL (https://github.com/apps/<slug>/installations/new) if the connector has the proper configuration The Github application needs to have 'Request user authorization (OAuth) during installation' enabled to perform the proper auth loop.
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
    ///         .connectors
    ///         .get_auth_url(
    ///             &"connector_id_or_name".to_string(),
    ///             &GetAuthURLQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_auth_url(
        &self,
        connector_id_or_name: &str,
        request: &GetAuthUrlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AuthUrlResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/connectors/{}/auth_url", connector_id_or_name),
                None,
                QueryBuilder::new()
                    .serialize("app_return_url", request.app_return_url.clone())
                    .serialize("method_type", request.method_type.clone())
                    .serialize("credentials_name", request.credentials_name.clone())
                    .serialize("credentials_title", request.credentials_title.clone())
                    .bool(
                        "github_installation_link",
                        request.github_installation_link.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Get the authentication schema for a connector. Returns the list of supported authentication methods and their required headers.
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
    ///         .connectors
    ///         .get_authentication_methods(&"connector_id_or_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_authentication_methods(
        &self,
        connector_id_or_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<PublicAuthenticationMethod>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/connectors/{}/authentication_methods",
                    connector_id_or_name
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// List all credentials configured at the organization level for a given connector.
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
    ///         .connectors
    ///         .list_organization_credentials(
    ///             &"connector_id_or_name".to_string(),
    ///             &ListOrganizationCredentialsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_organization_credentials(
        &self,
        connector_id_or_name: &str,
        request: &ListOrganizationCredentialsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CredentialsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/connectors/{}/organization/credentials",
                    connector_id_or_name
                ),
                None,
                QueryBuilder::new()
                    .serialize("auth_type", request.auth_type.clone())
                    .bool("fetch_default", request.fetch_default.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List all tools available for an MCP connector.
    ///
    /// # Arguments
    ///
    /// * `pretty` - Return a simplified payload with only name, description, annotations, and a compact inputSchema.
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
    ///         .connectors
    ///         .list_tools(
    ///             &"connector_id_or_name".to_string(),
    ///             &ListToolsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_tools(
        &self,
        connector_id_or_name: &str,
        request: &ListToolsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListToolsConnectorsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/connectors/{}/tools", connector_id_or_name),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .bool("refresh", request.refresh.clone())
                    .bool("pretty", request.pretty.clone())
                    .serialize("credentials_name", request.credentials_name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Call a tool on an MCP connector.
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
    ///         .connectors
    ///         .call_tool(
    ///             &"connector_id_or_name".to_string(),
    ///             &"tool_name".to_string(),
    ///             &ConnectorCallToolRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn call_tool(
        &self,
        connector_id_or_name: &str,
        tool_name: &str,
        request: &ConnectorCallToolRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConnectorToolCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/connectors/{}/tools/{}/call",
                    connector_id_or_name, tool_name
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("credentials_name", request.credentials_name.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List all credentials configured at the user level for a given connector.
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
    ///         .connectors
    ///         .list_user_credentials(
    ///             &"connector_id_or_name".to_string(),
    ///             &ListUserCredentialsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_user_credentials(
        &self,
        connector_id_or_name: &str,
        request: &ListUserCredentialsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CredentialsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/connectors/{}/user/credentials", connector_id_or_name),
                None,
                QueryBuilder::new()
                    .serialize("auth_type", request.auth_type.clone())
                    .bool("fetch_default", request.fetch_default.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete all credentials configured at the user level for a given connector.
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
    ///         .connectors
    ///         .delete_all_user_credentials(&"connector_id_or_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_all_user_credentials(
        &self,
        connector_id_or_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/connectors/{}/user/credentials", connector_id_or_name),
                None,
                None,
                options,
            )
            .await
    }

    /// List all credentials configured at the workspace level for a given connector.
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
    ///         .connectors
    ///         .list_workspace_credentials(
    ///             &"connector_id_or_name".to_string(),
    ///             &ListWorkspaceCredentialsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_workspace_credentials(
        &self,
        connector_id_or_name: &str,
        request: &ListWorkspaceCredentialsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CredentialsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/connectors/{}/workspace/credentials",
                    connector_id_or_name
                ),
                None,
                QueryBuilder::new()
                    .serialize("auth_type", request.auth_type.clone())
                    .bool("fetch_default", request.fetch_default.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create consumer credentials for a given connector and consumer scope (workspace, user)
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
    ///         .connectors
    ///         .create_credentials(
    ///             &"connector_id_or_name".to_string(),
    ///             &CreateCredentialsConnectorsRequestConsumerScope::User,
    ///             &CredentialsCreateOrUpdate {
    ///                 name: "name".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_credentials(
        &self,
        connector_id_or_name: &str,
        consumer_scope: &CreateCredentialsConnectorsRequestConsumerScope,
        request: &CredentialsCreateOrUpdate,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/connectors/{}/{}/credentials",
                    connector_id_or_name, consumer_scope
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create or update consumer credentials for a given connector.
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
    ///         .connectors
    ///         .update_credentials(
    ///             &"connector_id_or_name".to_string(),
    ///             &UpdateCredentialsConnectorsRequestConsumerScope::User,
    ///             &CredentialsCreateOrUpdate {
    ///                 name: "name".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_credentials(
        &self,
        connector_id_or_name: &str,
        consumer_scope: &UpdateCredentialsConnectorsRequestConsumerScope,
        request: &CredentialsCreateOrUpdate,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/connectors/{}/{}/credentials",
                    connector_id_or_name, consumer_scope
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete connector credentials for a given consumer.
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
    ///         .connectors
    ///         .delete_credentials(
    ///             &"connector_id_or_name".to_string(),
    ///             &DeleteCredentialsConnectorsRequestConsumerScope::User,
    ///             &"credentials_name".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_credentials(
        &self,
        connector_id_or_name: &str,
        consumer_scope: &DeleteCredentialsConnectorsRequestConsumerScope,
        credentials_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/connectors/{}/{}/credentials/{}",
                    connector_id_or_name, consumer_scope, credentials_name
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a connector by its ID.
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
    ///         .connectors
    ///         .delete(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        connector_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/connectors/{}#id", connector_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a connector by its ID.
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
    ///         .connectors
    ///         .update(
    ///             &"connector_id".to_string(),
    ///             &UpdateConnectorsRequestBody::HTTP {
    ///                 data: UpdateHTTPConnectorRequest {
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        connector_id: &str,
        request: &UpdateConnectorsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<Connector, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/connectors/{}#id", connector_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Transfers ownership of a private user-owned connector to the current organization, making it available to all organization members. The creator can later revert this via the unshare endpoint. Any authentication flows that rely on the original owner's identity (e.g. OAuth on-behalf-of) will be affected and must be reconfigured after sharing. Requires the ShareConnectorToOrg organization permission.
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
    ///         .connectors
    ///         .share_to_organization(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn share_to_organization(
        &self,
        connector_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/connectors/{}/organization/share", connector_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Reverts an organization-shared connector back to a private, creator-owned connector. Organization-scoped connections and other members' connections are removed; the creator's own connection is preserved. Requires the ShareConnectorToOrg organization permission.
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
    ///         .connectors
    ///         .unshare_from_organization(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn unshare_from_organization(
        &self,
        connector_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/connectors/{}/organization/share", connector_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Transfers ownership of a private user-owned connector to the current workspace, making it available to all workspace members. The creator can later revert this via the unshare endpoint. Any authentication flows that rely on the original owner's identity (e.g. OAuth on-behalf-of) will be affected and must be reconfigured after sharing. Only the connector's creator can call this endpoint. Requires the ShareConnectorToWorkspace workspace permission.
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
    ///         .connectors
    ///         .share(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn share(
        &self,
        connector_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/connectors/{}/share", connector_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Reverts a workspace-shared connector back to a private, creator-owned connector. Workspace-scoped connections and other members' connections are removed; the creator's own connection is preserved. Only the connector's creator can call this endpoint. Requires the ShareConnectorToWorkspace workspace permission.
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
    ///         .connectors
    ///         .unshare(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn unshare(
        &self,
        connector_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/connectors/{}/share", connector_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Enable a connector for the consumer.
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
    ///         .connectors
    ///         .activate_for_consumer(
    ///             &"connector_id".to_string(),
    ///             &ActivateForConsumerConnectorsRequestConsumerScope::User,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn activate_for_consumer(
        &self,
        connector_id: &str,
        consumer_scope: &ActivateForConsumerConnectorsRequestConsumerScope,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/connectors/{}/{}/activate", connector_id, consumer_scope),
                None,
                None,
                options,
            )
            .await
    }

    /// Disable a connector for the calling consumer only.
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
    ///         .connectors
    ///         .deactivate_for_consumer(
    ///             &"connector_id".to_string(),
    ///             &DeactivateForConsumerConnectorsRequestConsumerScope::User,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn deactivate_for_consumer(
        &self,
        connector_id: &str,
        consumer_scope: &DeactivateForConsumerConnectorsRequestConsumerScope,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/connectors/{}/{}/deactivate",
                    connector_id, consumer_scope
                ),
                None,
                None,
                options,
            )
            .await
    }
}
