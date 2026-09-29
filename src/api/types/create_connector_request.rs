pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateConnectorRequest {
    /// Protocol of the connector. Only 'mcp' is supported on the public endpoint; creating HTTP connectors here is explicitly refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<CreateConnectorRequestProtocol>,
    /// The name of the connector. Should be 64 char length maximum, alphanumeric, only underscores/dashes.
    #[serde(default)]
    pub name: String,
    /// Optional human-readable title for the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The description of the connector.
    #[serde(default)]
    pub description: String,
    /// The optional url of the icon you want to associate to the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Visibility of the connector. Use 'shared_workspace' for workspace scoped connectors, or 'private' for private connectors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<PublicResourceVisibility>,
    /// The url of the MCP server.
    #[serde(default)]
    pub server: String,
    /// Optional organization-level headers to be sent with the request to the mcp server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, serde_json::Value>>,
    /// Optional connector-wide headers, keyed by header name, set at creation and applied to every credential. Secret values are encrypted at rest and never returned in clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_headers: Option<HashMap<String, GlobalHeaderValue>>,
    /// Optional additional authentication data for the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_data: Option<AuthData>,
    /// Optional OAuth2 authorization server metadata (authorization_endpoint, token_endpoint, etc.). When provided, skips .well-known discovery and uses these endpoints directly.
    #[serde(rename = "oauth2_server_metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
    /// Optional URL to fetch OAuth2 authorization server metadata from (RFC 8414). When provided, the metadata is fetched from this URL and used instead of .well-known discovery. Mutually exclusive with oauth2_server_metadata.
    #[serde(rename = "oauth2_server_metadata_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2server_metadata_url: Option<String>,
    /// Optional system prompt for the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
}

impl CreateConnectorRequest {
    pub fn builder() -> CreateConnectorRequestBuilder {
        <CreateConnectorRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConnectorRequestBuilder {
    protocol: Option<CreateConnectorRequestProtocol>,
    name: Option<String>,
    title: Option<String>,
    description: Option<String>,
    icon_url: Option<String>,
    visibility: Option<PublicResourceVisibility>,
    server: Option<String>,
    headers: Option<HashMap<String, serde_json::Value>>,
    global_headers: Option<HashMap<String, GlobalHeaderValue>>,
    auth_data: Option<AuthData>,
    oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
    oauth2server_metadata_url: Option<String>,
    system_prompt: Option<String>,
}

impl CreateConnectorRequestBuilder {
    pub fn protocol(mut self, value: CreateConnectorRequestProtocol) -> Self {
        self.protocol = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: PublicResourceVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn server(mut self, value: impl Into<String>) -> Self {
        self.server = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn global_headers(mut self, value: HashMap<String, GlobalHeaderValue>) -> Self {
        self.global_headers = Some(value);
        self
    }

    pub fn auth_data(mut self, value: AuthData) -> Self {
        self.auth_data = Some(value);
        self
    }

    pub fn oauth2server_metadata(mut self, value: ExtendedOAuthServerMetadata) -> Self {
        self.oauth2server_metadata = Some(value);
        self
    }

    pub fn oauth2server_metadata_url(mut self, value: impl Into<String>) -> Self {
        self.oauth2server_metadata_url = Some(value.into());
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConnectorRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateConnectorRequestBuilder::name)
    /// - [`description`](CreateConnectorRequestBuilder::description)
    /// - [`server`](CreateConnectorRequestBuilder::server)
    pub fn build(self) -> Result<CreateConnectorRequest, BuildError> {
        Ok(CreateConnectorRequest {
            protocol: self.protocol,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            title: self.title,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            icon_url: self.icon_url,
            visibility: self.visibility,
            server: self
                .server
                .ok_or_else(|| BuildError::missing_field("server"))?,
            headers: self.headers,
            global_headers: self.global_headers,
            auth_data: self.auth_data,
            oauth2server_metadata: self.oauth2server_metadata,
            oauth2server_metadata_url: self.oauth2server_metadata_url,
            system_prompt: self.system_prompt,
        })
    }
}
