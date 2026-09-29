pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<ConversationStreamRequestAgentVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_execution: Option<ConversationStreamRequestHandoffExecution>,
    pub inputs: ConversationInputs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ConversationStreamRequestToolsItem>>,
}

impl ConversationStreamRequest {
    pub fn builder() -> ConversationStreamRequestBuilder {
        <ConversationStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationStreamRequestBuilder {
    agent_id: Option<String>,
    agent_version: Option<ConversationStreamRequestAgentVersion>,
    completion_args: Option<CompletionArgs>,
    description: Option<String>,
    guardrails: Option<Vec<GuardrailConfig>>,
    handoff_execution: Option<ConversationStreamRequestHandoffExecution>,
    inputs: Option<ConversationInputs>,
    instructions: Option<String>,
    metadata: Option<MetadataDict>,
    model: Option<String>,
    name: Option<String>,
    store: Option<bool>,
    stream: Option<bool>,
    tools: Option<Vec<ConversationStreamRequestToolsItem>>,
}

impl ConversationStreamRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: ConversationStreamRequestAgentVersion) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn completion_args(mut self, value: CompletionArgs) -> Self {
        self.completion_args = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn guardrails(mut self, value: Vec<GuardrailConfig>) -> Self {
        self.guardrails = Some(value);
        self
    }

    pub fn handoff_execution(mut self, value: ConversationStreamRequestHandoffExecution) -> Self {
        self.handoff_execution = Some(value);
        self
    }

    pub fn inputs(mut self, value: ConversationInputs) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: MetadataDict) -> Self {
        self.metadata = Some(value);
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

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<ConversationStreamRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inputs`](ConversationStreamRequestBuilder::inputs)
    pub fn build(self) -> Result<ConversationStreamRequest, BuildError> {
        Ok(ConversationStreamRequest {
            agent_id: self.agent_id,
            agent_version: self.agent_version,
            completion_args: self.completion_args,
            description: self.description,
            guardrails: self.guardrails,
            handoff_execution: self.handoff_execution,
            inputs: self
                .inputs
                .ok_or_else(|| BuildError::missing_field("inputs"))?,
            instructions: self.instructions,
            metadata: self.metadata,
            model: self.model,
            name: self.name,
            store: self.store,
            stream: self.stream,
            tools: self.tools,
        })
    }
}
