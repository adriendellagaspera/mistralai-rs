pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Agent {
    /// Instruction prompt the model will follow during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// List of tools which are available to the model during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AgentToolsItem>>,
    /// Completion arguments that will be used to generate assistant responses. Can be overridden at each message request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoffs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<AgentObject>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub versions: Vec<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub deployment_chat: bool,
    #[serde(default)]
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_message: Option<String>,
}

impl Agent {
    pub fn builder() -> AgentBuilder {
        <AgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentBuilder {
    instructions: Option<String>,
    tools: Option<Vec<AgentToolsItem>>,
    completion_args: Option<CompletionArgs>,
    guardrails: Option<Vec<GuardrailConfig>>,
    model: Option<String>,
    name: Option<String>,
    description: Option<String>,
    handoffs: Option<Vec<String>>,
    metadata: Option<MetadataDict>,
    object: Option<AgentObject>,
    id: Option<String>,
    version: Option<i64>,
    versions: Option<Vec<i64>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    deployment_chat: Option<bool>,
    source: Option<String>,
    version_message: Option<String>,
}

impl AgentBuilder {
    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<AgentToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn completion_args(mut self, value: CompletionArgs) -> Self {
        self.completion_args = Some(value);
        self
    }

    pub fn guardrails(mut self, value: Vec<GuardrailConfig>) -> Self {
        self.guardrails = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
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

    pub fn handoffs(mut self, value: Vec<String>) -> Self {
        self.handoffs = Some(value);
        self
    }

    pub fn metadata(mut self, value: MetadataDict) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn object(mut self, value: AgentObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn versions(mut self, value: Vec<i64>) -> Self {
        self.versions = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn deployment_chat(mut self, value: bool) -> Self {
        self.deployment_chat = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn version_message(mut self, value: impl Into<String>) -> Self {
        self.version_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Agent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](AgentBuilder::model)
    /// - [`name`](AgentBuilder::name)
    /// - [`id`](AgentBuilder::id)
    /// - [`version`](AgentBuilder::version)
    /// - [`versions`](AgentBuilder::versions)
    /// - [`created_at`](AgentBuilder::created_at)
    /// - [`updated_at`](AgentBuilder::updated_at)
    /// - [`deployment_chat`](AgentBuilder::deployment_chat)
    /// - [`source`](AgentBuilder::source)
    pub fn build(self) -> Result<Agent, BuildError> {
        Ok(Agent {
            instructions: self.instructions,
            tools: self.tools,
            completion_args: self.completion_args,
            guardrails: self.guardrails,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            handoffs: self.handoffs,
            metadata: self.metadata,
            object: self.object,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            versions: self
                .versions
                .ok_or_else(|| BuildError::missing_field("versions"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            deployment_chat: self
                .deployment_chat
                .ok_or_else(|| BuildError::missing_field("deployment_chat"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            version_message: self.version_message,
        })
    }
}
