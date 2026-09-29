pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAgentRequest {
    /// Completion arguments that will be used to generate assistant responses. Can be overridden at each message request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoffs: Option<Vec<String>>,
    /// Instruction prompt the model will follow during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub name: String,
    /// List of tools which are available to the model during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<CreateAgentRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_message: Option<String>,
}

impl CreateAgentRequest {
    pub fn builder() -> CreateAgentRequestBuilder {
        <CreateAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRequestBuilder {
    completion_args: Option<CompletionArgs>,
    description: Option<String>,
    guardrails: Option<Vec<GuardrailConfig>>,
    handoffs: Option<Vec<String>>,
    instructions: Option<String>,
    metadata: Option<MetadataDict>,
    model: Option<String>,
    name: Option<String>,
    tools: Option<Vec<CreateAgentRequestToolsItem>>,
    version_message: Option<String>,
}

impl CreateAgentRequestBuilder {
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

    pub fn handoffs(mut self, value: Vec<String>) -> Self {
        self.handoffs = Some(value);
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

    pub fn tools(mut self, value: Vec<CreateAgentRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn version_message(mut self, value: impl Into<String>) -> Self {
        self.version_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](CreateAgentRequestBuilder::model)
    /// - [`name`](CreateAgentRequestBuilder::name)
    pub fn build(self) -> Result<CreateAgentRequest, BuildError> {
        Ok(CreateAgentRequest {
            completion_args: self.completion_args,
            description: self.description,
            guardrails: self.guardrails,
            handoffs: self.handoffs,
            instructions: self.instructions,
            metadata: self.metadata,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tools: self.tools,
            version_message: self.version_message,
        })
    }
}
