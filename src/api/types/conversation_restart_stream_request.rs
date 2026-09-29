pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationRestartStreamRequest {
    /// Specific version of the agent to use when restarting. If not provided, uses the current version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<ConversationRestartStreamRequestAgentVersion>,
    /// Completion arguments that will be used to generate assistant responses. Can be overridden at each message request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(default)]
    pub from_entry_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_execution: Option<ConversationRestartStreamRequestHandoffExecution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<ConversationInputs>,
    /// Custom metadata for the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    /// Whether to store the results into our servers or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl ConversationRestartStreamRequest {
    pub fn builder() -> ConversationRestartStreamRequestBuilder {
        <ConversationRestartStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationRestartStreamRequestBuilder {
    agent_version: Option<ConversationRestartStreamRequestAgentVersion>,
    completion_args: Option<CompletionArgs>,
    from_entry_id: Option<String>,
    guardrails: Option<Vec<GuardrailConfig>>,
    handoff_execution: Option<ConversationRestartStreamRequestHandoffExecution>,
    inputs: Option<ConversationInputs>,
    metadata: Option<MetadataDict>,
    store: Option<bool>,
    stream: Option<bool>,
}

impl ConversationRestartStreamRequestBuilder {
    pub fn agent_version(mut self, value: ConversationRestartStreamRequestAgentVersion) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn completion_args(mut self, value: CompletionArgs) -> Self {
        self.completion_args = Some(value);
        self
    }

    pub fn from_entry_id(mut self, value: impl Into<String>) -> Self {
        self.from_entry_id = Some(value.into());
        self
    }

    pub fn guardrails(mut self, value: Vec<GuardrailConfig>) -> Self {
        self.guardrails = Some(value);
        self
    }

    pub fn handoff_execution(
        mut self,
        value: ConversationRestartStreamRequestHandoffExecution,
    ) -> Self {
        self.handoff_execution = Some(value);
        self
    }

    pub fn inputs(mut self, value: ConversationInputs) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn metadata(mut self, value: MetadataDict) -> Self {
        self.metadata = Some(value);
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

    /// Consumes the builder and constructs a [`ConversationRestartStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_entry_id`](ConversationRestartStreamRequestBuilder::from_entry_id)
    pub fn build(self) -> Result<ConversationRestartStreamRequest, BuildError> {
        Ok(ConversationRestartStreamRequest {
            agent_version: self.agent_version,
            completion_args: self.completion_args,
            from_entry_id: self
                .from_entry_id
                .ok_or_else(|| BuildError::missing_field("from_entry_id"))?,
            guardrails: self.guardrails,
            handoff_execution: self.handoff_execution,
            inputs: self.inputs,
            metadata: self.metadata,
            store: self.store,
            stream: self.stream,
        })
    }
}
