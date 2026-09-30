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
    ///         .connector_list_v1(
    ///             &ConnectorListV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_list_v1(
        &self,
        request: &ConnectorListV1QueryRequest,
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

    /// Create a new MCP connector. You can customize its visibility, url and auth type.
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
    ///         .connector_create_v1(
    ///             &CreateConnectorRequest {
    ///                 description: "description".to_string(),
    ///                 name: "name".to_string(),
    ///                 server: "server".to_string(),
    ///                 auth_data: None,
    ///                 global_headers: None,
    ///                 headers: None,
    ///                 icon_url: None,
    ///                 oauth2server_metadata: None,
    ///                 oauth2server_metadata_url: None,
    ///                 protocol: None,
    ///                 system_prompt: None,
    ///                 title: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_create_v1(
        &self,
        request: &CreateConnectorRequest,
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
    /// * `fetch_customer_data` - Fetch the customer data associated with the connector (e.g. customer secrets / config).
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
    ///         .connector_get_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &ConnectorGetV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_get_v1(
        &self,
        connector_id_or_name: &str,
        request: &ConnectorGetV1QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Connector, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/connectors/{}#idOrName", connector_id_or_name),
                None,
                QueryBuilder::new()
                    .bool("fetch_user_data", request.fetch_user_data.clone())
                    .bool("fetch_customer_data", request.fetch_customer_data.clone())
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
    ///         .connector_get_auth_url_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &ConnectorGetAuthURLV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_get_auth_url_v1(
        &self,
        connector_id_or_name: &str,
        request: &ConnectorGetAuthUrlV1QueryRequest,
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
    ///         .connector_get_authentication_methods_v1(&"connector_id_or_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn connector_get_authentication_methods_v1(
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
    ///         .connector_list_organization_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &ConnectorListOrganizationCredentialsV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_list_organization_credentials_v1(
        &self,
        connector_id_or_name: &str,
        request: &ConnectorListOrganizationCredentialsV1QueryRequest,
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

    /// Create or update credentials at the organization level for a given connector.
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
    ///         .connector_create_or_update_organization_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &CredentialsCreateOrUpdate {
    ///                 name: "name".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_create_or_update_organization_credentials_v1(
        &self,
        connector_id_or_name: &str,
        request: &CredentialsCreateOrUpdate,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/connectors/{}/organization/credentials",
                    connector_id_or_name
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete credentials at the organization level for a given connector.
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
    ///         .connector_delete_organization_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &"credentials_name".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_delete_organization_credentials_v1(
        &self,
        connector_id_or_name: &str,
        credentials_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/connectors/{}/organization/credentials/{}",
                    connector_id_or_name, credentials_name
                ),
                None,
                None,
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
    ///         .connector_list_tools_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &ConnectorListToolsV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_list_tools_v1(
        &self,
        connector_id_or_name: &str,
        request: &ConnectorListToolsV1QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConnectorListToolsV1ConnectorsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/connectors/{}/tools", connector_id_or_name),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
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
    ///         .connector_call_tool_v1(
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
    pub async fn connector_call_tool_v1(
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
    ///         .connector_list_user_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &ConnectorListUserCredentialsV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_list_user_credentials_v1(
        &self,
        connector_id_or_name: &str,
        request: &ConnectorListUserCredentialsV1QueryRequest,
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

    /// Create or update credentials at the user level for a given connector.
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
    ///         .connector_create_or_update_user_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &CredentialsCreateOrUpdate {
    ///                 name: "name".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_create_or_update_user_credentials_v1(
        &self,
        connector_id_or_name: &str,
        request: &CredentialsCreateOrUpdate,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/connectors/{}/user/credentials", connector_id_or_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
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
    ///         .connector_delete_all_user_credentials_v1(&"connector_id_or_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn connector_delete_all_user_credentials_v1(
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

    /// Delete credentials at the user level for a given connector.
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
    ///         .connector_delete_user_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &"credentials_name".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_delete_user_credentials_v1(
        &self,
        connector_id_or_name: &str,
        credentials_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/connectors/{}/user/credentials/{}",
                    connector_id_or_name, credentials_name
                ),
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
    ///         .connector_list_workspace_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &ConnectorListWorkspaceCredentialsV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_list_workspace_credentials_v1(
        &self,
        connector_id_or_name: &str,
        request: &ConnectorListWorkspaceCredentialsV1QueryRequest,
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

    /// Create or update credentials at the workspace level for a given connector.
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
    ///         .connector_create_or_update_workspace_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &CredentialsCreateOrUpdate {
    ///                 name: "name".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_create_or_update_workspace_credentials_v1(
        &self,
        connector_id_or_name: &str,
        request: &CredentialsCreateOrUpdate,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/connectors/{}/workspace/credentials",
                    connector_id_or_name
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete credentials at the workspace level for a given connector.
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
    ///         .connector_delete_workspace_credentials_v1(
    ///             &"connector_id_or_name".to_string(),
    ///             &"credentials_name".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_delete_workspace_credentials_v1(
        &self,
        connector_id_or_name: &str,
        credentials_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<MessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/connectors/{}/workspace/credentials/{}",
                    connector_id_or_name, credentials_name
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
    ///         .connector_delete_v1(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn connector_delete_v1(
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
    ///         .connector_update_v1(
    ///             &"connector_id".to_string(),
    ///             &UpdateConnectorRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_update_v1(
        &self,
        connector_id: &str,
        request: &UpdateConnectorRequest,
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
    ///         .connector_share_v1(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn connector_share_v1(
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
    ///         .connector_unshare_v1(&"connector_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn connector_unshare_v1(
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
    ///         .connector_activate_for_consumer_v1(
    ///             &"connector_id".to_string(),
    ///             &ConnectorActivateForConsumerV1ConnectorsRequestConsumerScope::User,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_activate_for_consumer_v1(
        &self,
        connector_id: &str,
        consumer_scope: &ConnectorActivateForConsumerV1ConnectorsRequestConsumerScope,
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
    ///         .connector_deactivate_for_consumer_v1(
    ///             &"connector_id".to_string(),
    ///             &ConnectorDeactivateForConsumerV1ConnectorsRequestConsumerScope::User,
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connector_deactivate_for_consumer_v1(
        &self,
        connector_id: &str,
        consumer_scope: &ConnectorDeactivateForConsumerV1ConnectorsRequestConsumerScope,
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
