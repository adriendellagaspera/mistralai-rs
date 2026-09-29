pub use crate::prelude::*;

/// Capabilities that a server may support.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions: Option<CompletionsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<ServerTasksCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ServerCapabilities {
    pub fn builder() -> ServerCapabilitiesBuilder {
        <ServerCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServerCapabilitiesBuilder {
    experimental: Option<HashMap<String, Option<HashMap<String, serde_json::Value>>>>,
    logging: Option<LoggingCapability>,
    prompts: Option<PromptsCapability>,
    resources: Option<ResourcesCapability>,
    tools: Option<ToolsCapability>,
    completions: Option<CompletionsCapability>,
    tasks: Option<ServerTasksCapability>,
}

impl ServerCapabilitiesBuilder {
    pub fn experimental(
        mut self,
        value: HashMap<String, Option<HashMap<String, serde_json::Value>>>,
    ) -> Self {
        self.experimental = Some(value);
        self
    }

    pub fn logging(mut self, value: LoggingCapability) -> Self {
        self.logging = Some(value);
        self
    }

    pub fn prompts(mut self, value: PromptsCapability) -> Self {
        self.prompts = Some(value);
        self
    }

    pub fn resources(mut self, value: ResourcesCapability) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn tools(mut self, value: ToolsCapability) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn completions(mut self, value: CompletionsCapability) -> Self {
        self.completions = Some(value);
        self
    }

    pub fn tasks(mut self, value: ServerTasksCapability) -> Self {
        self.tasks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServerCapabilities`].
    pub fn build(self) -> Result<ServerCapabilities, BuildError> {
        Ok(ServerCapabilities {
            experimental: self.experimental,
            logging: self.logging,
            prompts: self.prompts,
            resources: self.resources,
            tools: self.tools,
            completions: self.completions,
            tasks: self.tasks,
            extra: Default::default(),
        })
    }
}
