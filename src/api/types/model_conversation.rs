pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelConversation {
    /// Completion arguments that will be used to generate assistant responses. Can be overridden at each message request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Description of the what the conversation is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    #[serde(default)]
    pub id: String,
    /// Instruction prompt the model will follow during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Custom metadata for the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(default)]
    pub model: String,
    /// Name given to the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ModelConversationObject>,
    /// List of tools which are available to the model during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ModelConversationToolsItem>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ModelConversation {
    pub fn builder() -> ModelConversationBuilder {
        <ModelConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelConversationBuilder {
    completion_args: Option<CompletionArgs>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    guardrails: Option<Vec<GuardrailConfig>>,
    id: Option<String>,
    instructions: Option<String>,
    metadata: Option<MetadataDict>,
    model: Option<String>,
    name: Option<String>,
    object: Option<ModelConversationObject>,
    tools: Option<Vec<ModelConversationToolsItem>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ModelConversationBuilder {
    pub fn completion_args(mut self, value: CompletionArgs) -> Self {
        self.completion_args = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn object(mut self, value: ModelConversationObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<ModelConversationToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelConversation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ModelConversationBuilder::created_at)
    /// - [`id`](ModelConversationBuilder::id)
    /// - [`model`](ModelConversationBuilder::model)
    /// - [`updated_at`](ModelConversationBuilder::updated_at)
    pub fn build(self) -> Result<ModelConversation, BuildError> {
        Ok(ModelConversation {
            completion_args: self.completion_args,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            guardrails: self.guardrails,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            instructions: self.instructions,
            metadata: self.metadata,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            name: self.name,
            object: self.object,
            tools: self.tools,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
