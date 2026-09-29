pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationRequest {
    pub inputs: ConversationInputs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_execution: Option<ConversationRequestHandoffExecution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ConversationRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<ConversationRequestAgentVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl ConversationRequest {
    pub fn builder() -> ConversationRequestBuilder {
        <ConversationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationRequestBuilder {
    inputs: Option<ConversationInputs>,
    stream: Option<bool>,
    store: Option<bool>,
    handoff_execution: Option<ConversationRequestHandoffExecution>,
    instructions: Option<String>,
    tools: Option<Vec<ConversationRequestToolsItem>>,
    completion_args: Option<CompletionArgs>,
    guardrails: Option<Vec<GuardrailConfig>>,
    name: Option<String>,
    description: Option<String>,
    metadata: Option<MetadataDict>,
    agent_id: Option<String>,
    agent_version: Option<ConversationRequestAgentVersion>,
    model: Option<String>,
}

impl ConversationRequestBuilder {
    pub fn inputs(mut self, value: ConversationInputs) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
        self
    }

    pub fn handoff_execution(mut self, value: ConversationRequestHandoffExecution) -> Self {
        self.handoff_execution = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<ConversationRequestToolsItem>) -> Self {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: MetadataDict) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: ConversationRequestAgentVersion) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inputs`](ConversationRequestBuilder::inputs)
    pub fn build(self) -> Result<ConversationRequest, BuildError> {
        Ok(ConversationRequest {
            inputs: self
                .inputs
                .ok_or_else(|| BuildError::missing_field("inputs"))?,
            stream: self.stream,
            store: self.store,
            handoff_execution: self.handoff_execution,
            instructions: self.instructions,
            tools: self.tools,
            completion_args: self.completion_args,
            guardrails: self.guardrails,
            name: self.name,
            description: self.description,
            metadata: self.metadata,
            agent_id: self.agent_id,
            agent_version: self.agent_version,
            model: self.model,
        })
    }
}
