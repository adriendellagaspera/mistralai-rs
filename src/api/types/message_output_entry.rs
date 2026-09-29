pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOutputEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub content: MessageOutputEntryContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<MessageOutputEntryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageOutputEntryRole>,
}

impl MessageOutputEntry {
    pub fn builder() -> MessageOutputEntryBuilder {
        <MessageOutputEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageOutputEntryBuilder {
    agent_id: Option<String>,
    completed_at: Option<DateTime<FixedOffset>>,
    content: Option<MessageOutputEntryContent>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    model: Option<String>,
    object: Option<MessageOutputEntryObject>,
    role: Option<MessageOutputEntryRole>,
}

impl MessageOutputEntryBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn content(mut self, value: MessageOutputEntryContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn object(mut self, value: MessageOutputEntryObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn role(mut self, value: MessageOutputEntryRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageOutputEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](MessageOutputEntryBuilder::content)
    pub fn build(self) -> Result<MessageOutputEntry, BuildError> {
        Ok(MessageOutputEntry {
            agent_id: self.agent_id,
            completed_at: self.completed_at,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            created_at: self.created_at,
            id: self.id,
            model: self.model,
            object: self.object,
            role: self.role,
        })
    }
}
