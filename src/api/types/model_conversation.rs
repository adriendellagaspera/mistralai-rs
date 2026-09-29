pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelConversation {
    /// Instruction prompt the model will follow during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// List of tools which are available to the model during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ModelConversationToolsItem>>,
    /// Completion arguments that will be used to generate assistant responses. Can be overridden at each message request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailConfig>>,
    /// Name given to the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the what the conversation is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Custom metadata for the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ModelConversationObject>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub model: String,
}

impl ModelConversation {
    pub fn builder() -> ModelConversationBuilder {
        <ModelConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelConversationBuilder {
    instructions: Option<String>,
    tools: Option<Vec<ModelConversationToolsItem>>,
    completion_args: Option<CompletionArgs>,
    guardrails: Option<Vec<GuardrailConfig>>,
    name: Option<String>,
    description: Option<String>,
    metadata: Option<MetadataDict>,
    object: Option<ModelConversationObject>,
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    model: Option<String>,
}

impl ModelConversationBuilder {
    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<ModelConversationToolsItem>) -> Self {
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

    pub fn object(mut self, value: ModelConversationObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ModelConversation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ModelConversationBuilder::id)
    /// - [`created_at`](ModelConversationBuilder::created_at)
    /// - [`updated_at`](ModelConversationBuilder::updated_at)
    /// - [`model`](ModelConversationBuilder::model)
    pub fn build(self) -> Result<ModelConversation, BuildError> {
        Ok(ModelConversation {
            instructions: self.instructions,
            tools: self.tools,
            completion_args: self.completion_args,
            guardrails: self.guardrails,
            name: self.name,
            description: self.description,
            metadata: self.metadata,
            object: self.object,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
        })
    }
}
