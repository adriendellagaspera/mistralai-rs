pub use crate::prelude::*;

/// Representation of an input message inside the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageInputEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub content: MessageInputEntryContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<MessageInputEntryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<bool>,
    pub role: MessageInputEntryRole,
}

impl MessageInputEntry {
    pub fn builder() -> MessageInputEntryBuilder {
        <MessageInputEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageInputEntryBuilder {
    completed_at: Option<DateTime<FixedOffset>>,
    content: Option<MessageInputEntryContent>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<MessageInputEntryObject>,
    prefix: Option<bool>,
    role: Option<MessageInputEntryRole>,
}

impl MessageInputEntryBuilder {
    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn content(mut self, value: MessageInputEntryContent) -> Self {
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

    pub fn object(mut self, value: MessageInputEntryObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn prefix(mut self, value: bool) -> Self {
        self.prefix = Some(value);
        self
    }

    pub fn role(mut self, value: MessageInputEntryRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageInputEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](MessageInputEntryBuilder::content)
    /// - [`role`](MessageInputEntryBuilder::role)
    pub fn build(self) -> Result<MessageInputEntry, BuildError> {
        Ok(MessageInputEntry {
            completed_at: self.completed_at,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            created_at: self.created_at,
            id: self.id,
            object: self.object,
            prefix: self.prefix,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
