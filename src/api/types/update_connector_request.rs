pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateConnectorRequest {
    /// Optional human-readable title for the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The name of the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The optional url of the icon you want to associate to the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Optional system prompt for the connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<UpdateConnectorRequestProtocol>,
    /// New server url for your mcp connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// list of authentication methods to add to the connector or to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_methods: Option<Vec<AuthenticationMethodCreateOrUpdateRequest>>,
}

impl UpdateConnectorRequest {
    pub fn builder() -> UpdateConnectorRequestBuilder {
        <UpdateConnectorRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateConnectorRequestBuilder {
    title: Option<String>,
    name: Option<String>,
    description: Option<String>,
    icon_url: Option<String>,
    system_prompt: Option<String>,
    protocol: Option<UpdateConnectorRequestProtocol>,
    server: Option<String>,
    auth_methods: Option<Vec<AuthenticationMethodCreateOrUpdateRequest>>,
}

impl UpdateConnectorRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn protocol(mut self, value: UpdateConnectorRequestProtocol) -> Self {
        self.protocol = Some(value);
        self
    }

    pub fn server(mut self, value: impl Into<String>) -> Self {
        self.server = Some(value.into());
        self
    }

    pub fn auth_methods(mut self, value: Vec<AuthenticationMethodCreateOrUpdateRequest>) -> Self {
        self.auth_methods = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateConnectorRequest`].
    pub fn build(self) -> Result<UpdateConnectorRequest, BuildError> {
        Ok(UpdateConnectorRequest {
            title: self.title,
            name: self.name,
            description: self.description,
            icon_url: self.icon_url,
            system_prompt: self.system_prompt,
            protocol: self.protocol,
            server: self.server,
            auth_methods: self.auth_methods,
        })
    }
}
