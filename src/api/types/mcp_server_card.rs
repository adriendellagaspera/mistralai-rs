pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServerCard {
    /// URL to the JSON schema definition
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Server identifier in reverse-DNS format with exactly one /
    #[serde(default)]
    pub name: String,
    /// Server version (semantic versioning preferred)
    #[serde(default)]
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<ServerCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "websiteUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<McpServerRepository>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<McpServerIcon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remotes: Option<Vec<McpServerRemote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<ClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<McpServerCardResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<McpServerCardTools>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<McpServerCardPrompts>,
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<McpServerCardMeta>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpServerCard {
    pub fn builder() -> McpServerCardBuilder {
        <McpServerCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerCardBuilder {
    schema: Option<String>,
    name: Option<String>,
    version: Option<String>,
    capabilities: Option<ServerCapabilities>,
    title: Option<String>,
    description: Option<String>,
    website_url: Option<String>,
    repository: Option<McpServerRepository>,
    icons: Option<Vec<McpServerIcon>>,
    remotes: Option<Vec<McpServerRemote>>,
    requires: Option<ClientCapabilities>,
    resources: Option<McpServerCardResources>,
    tools: Option<McpServerCardTools>,
    prompts: Option<McpServerCardPrompts>,
    meta: Option<McpServerCardMeta>,
}

impl McpServerCardBuilder {
    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.schema = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn capabilities(mut self, value: ServerCapabilities) -> Self {
        self.capabilities = Some(value);
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

    pub fn website_url(mut self, value: impl Into<String>) -> Self {
        self.website_url = Some(value.into());
        self
    }

    pub fn repository(mut self, value: McpServerRepository) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn icons(mut self, value: Vec<McpServerIcon>) -> Self {
        self.icons = Some(value);
        self
    }

    pub fn remotes(mut self, value: Vec<McpServerRemote>) -> Self {
        self.remotes = Some(value);
        self
    }

    pub fn requires(mut self, value: ClientCapabilities) -> Self {
        self.requires = Some(value);
        self
    }

    pub fn resources(mut self, value: McpServerCardResources) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn tools(mut self, value: McpServerCardTools) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn prompts(mut self, value: McpServerCardPrompts) -> Self {
        self.prompts = Some(value);
        self
    }

    pub fn meta(mut self, value: McpServerCardMeta) -> Self {
        self.meta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](McpServerCardBuilder::name)
    /// - [`version`](McpServerCardBuilder::version)
    pub fn build(self) -> Result<McpServerCard, BuildError> {
        Ok(McpServerCard {
            schema: self.schema,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            capabilities: self.capabilities,
            title: self.title,
            description: self.description,
            website_url: self.website_url,
            repository: self.repository,
            icons: self.icons,
            remotes: self.remotes,
            requires: self.requires,
            resources: self.resources,
            tools: self.tools,
            prompts: self.prompts,
            meta: self.meta,
            extra: Default::default(),
        })
    }
}
