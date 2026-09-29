pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connector {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub modified_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ConnectorProtocol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_card: Option<McpServerCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    pub owner_type: ConsumerType,
    pub visibility: ResourceVisibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ConnectorLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_auth_methods: Option<Vec<PublicAuthenticationMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_preferences: Option<Vec<ConnectionPreference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_credentials: Option<Vec<AuthenticationConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub private_tool_execution: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mistral: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_authenticated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ConnectorTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt_route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_config: Option<PublicConnectionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_env: Option<PublicExecutionEnv>,
}

impl Connector {
    pub fn builder() -> ConnectorBuilder {
        <ConnectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorBuilder {
    id: Option<String>,
    name: Option<String>,
    title: Option<String>,
    description: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    modified_at: Option<DateTime<FixedOffset>>,
    server: Option<String>,
    protocol: Option<ConnectorProtocol>,
    icon_url: Option<String>,
    server_card: Option<McpServerCard>,
    owner_id: Option<String>,
    owner_type: Option<ConsumerType>,
    visibility: Option<ResourceVisibility>,
    creator_id: Option<String>,
    locale: Option<ConnectorLocale>,
    system_prompt: Option<String>,
    supported_auth_methods: Option<Vec<PublicAuthenticationMethod>>,
    connection_preferences: Option<Vec<ConnectionPreference>>,
    connection_credentials: Option<Vec<AuthenticationConfiguration>>,
    active: Option<bool>,
    private_tool_execution: Option<bool>,
    mistral: Option<bool>,
    is_authenticated: Option<bool>,
    tools: Option<Vec<ConnectorTool>>,
    system_prompt_route: Option<String>,
    connection_config: Option<PublicConnectionConfig>,
    execution_env: Option<PublicExecutionEnv>,
}

impl ConnectorBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn server(mut self, value: impl Into<String>) -> Self {
        self.server = Some(value.into());
        self
    }

    pub fn protocol(mut self, value: ConnectorProtocol) -> Self {
        self.protocol = Some(value);
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn server_card(mut self, value: McpServerCard) -> Self {
        self.server_card = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn owner_type(mut self, value: ConsumerType) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn visibility(mut self, value: ResourceVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn locale(mut self, value: ConnectorLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn supported_auth_methods(mut self, value: Vec<PublicAuthenticationMethod>) -> Self {
        self.supported_auth_methods = Some(value);
        self
    }

    pub fn connection_preferences(mut self, value: Vec<ConnectionPreference>) -> Self {
        self.connection_preferences = Some(value);
        self
    }

    pub fn connection_credentials(mut self, value: Vec<AuthenticationConfiguration>) -> Self {
        self.connection_credentials = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn private_tool_execution(mut self, value: bool) -> Self {
        self.private_tool_execution = Some(value);
        self
    }

    pub fn mistral(mut self, value: bool) -> Self {
        self.mistral = Some(value);
        self
    }

    pub fn is_authenticated(mut self, value: bool) -> Self {
        self.is_authenticated = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<ConnectorTool>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn system_prompt_route(mut self, value: impl Into<String>) -> Self {
        self.system_prompt_route = Some(value.into());
        self
    }

    pub fn connection_config(mut self, value: PublicConnectionConfig) -> Self {
        self.connection_config = Some(value);
        self
    }

    pub fn execution_env(mut self, value: PublicExecutionEnv) -> Self {
        self.execution_env = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Connector`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConnectorBuilder::id)
    /// - [`name`](ConnectorBuilder::name)
    /// - [`description`](ConnectorBuilder::description)
    /// - [`created_at`](ConnectorBuilder::created_at)
    /// - [`modified_at`](ConnectorBuilder::modified_at)
    /// - [`owner_type`](ConnectorBuilder::owner_type)
    /// - [`visibility`](ConnectorBuilder::visibility)
    /// - [`private_tool_execution`](ConnectorBuilder::private_tool_execution)
    pub fn build(self) -> Result<Connector, BuildError> {
        Ok(Connector {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            title: self.title,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            server: self.server,
            protocol: self.protocol,
            icon_url: self.icon_url,
            server_card: self.server_card,
            owner_id: self.owner_id,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
            creator_id: self.creator_id,
            locale: self.locale,
            system_prompt: self.system_prompt,
            supported_auth_methods: self.supported_auth_methods,
            connection_preferences: self.connection_preferences,
            connection_credentials: self.connection_credentials,
            active: self.active,
            private_tool_execution: self
                .private_tool_execution
                .ok_or_else(|| BuildError::missing_field("private_tool_execution"))?,
            mistral: self.mistral,
            is_authenticated: self.is_authenticated,
            tools: self.tools,
            system_prompt_route: self.system_prompt_route,
            connection_config: self.connection_config,
            execution_env: self.execution_env,
        })
    }
}
