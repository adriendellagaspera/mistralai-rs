pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationRestartRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<ConversationInputs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Whether to store the results into our servers or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_execution: Option<ConversationRestartRequestHandoffExecution>,
    /// Completion arguments that will be used to generate assistant responses. Can be overridden at each message request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    /// Custom metadata for the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(default)]
    pub from_entry_id: String,
    /// Specific version of the agent to use when restarting. If not provided, uses the current version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<ConversationRestartRequestAgentVersion>,
}

impl ConversationRestartRequest {
    pub fn builder() -> ConversationRestartRequestBuilder {
        <ConversationRestartRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationRestartRequestBuilder {
    inputs: Option<ConversationInputs>,
    stream: Option<bool>,
    store: Option<bool>,
    handoff_execution: Option<ConversationRestartRequestHandoffExecution>,
    completion_args: Option<CompletionArgs>,
    guardrails: Option<Vec<GuardrailConfig>>,
    metadata: Option<MetadataDict>,
    from_entry_id: Option<String>,
    agent_version: Option<ConversationRestartRequestAgentVersion>,
}

impl ConversationRestartRequestBuilder {
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

    pub fn handoff_execution(mut self, value: ConversationRestartRequestHandoffExecution) -> Self {
        self.handoff_execution = Some(value);
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

    pub fn metadata(mut self, value: MetadataDict) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn from_entry_id(mut self, value: impl Into<String>) -> Self {
        self.from_entry_id = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: ConversationRestartRequestAgentVersion) -> Self {
        self.agent_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationRestartRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_entry_id`](ConversationRestartRequestBuilder::from_entry_id)
    pub fn build(self) -> Result<ConversationRestartRequest, BuildError> {
        Ok(ConversationRestartRequest {
            inputs: self.inputs,
            stream: self.stream,
            store: self.store,
            handoff_execution: self.handoff_execution,
            completion_args: self.completion_args,
            guardrails: self.guardrails,
            metadata: self.metadata,
            from_entry_id: self
                .from_entry_id
                .ok_or_else(|| BuildError::missing_field("from_entry_id"))?,
            agent_version: self.agent_version,
        })
    }
}
