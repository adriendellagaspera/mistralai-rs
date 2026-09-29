pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentConversation {
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
    pub object: Option<AgentConversationObject>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<AgentConversationAgentVersion>,
}

impl AgentConversation {
    pub fn builder() -> AgentConversationBuilder {
        <AgentConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentConversationBuilder {
    name: Option<String>,
    description: Option<String>,
    metadata: Option<MetadataDict>,
    object: Option<AgentConversationObject>,
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    agent_id: Option<String>,
    agent_version: Option<AgentConversationAgentVersion>,
}

impl AgentConversationBuilder {
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

    pub fn object(mut self, value: AgentConversationObject) -> Self {
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

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: AgentConversationAgentVersion) -> Self {
        self.agent_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentConversation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentConversationBuilder::id)
    /// - [`created_at`](AgentConversationBuilder::created_at)
    /// - [`updated_at`](AgentConversationBuilder::updated_at)
    /// - [`agent_id`](AgentConversationBuilder::agent_id)
    pub fn build(self) -> Result<AgentConversation, BuildError> {
        Ok(AgentConversation {
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
            agent_id: self
                .agent_id
                .ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_version: self.agent_version,
        })
    }
}
