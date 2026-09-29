pub use crate::prelude::*;

/// Retrieve all entries in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationHistory {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub entries: Vec<ConversationHistoryEntriesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ConversationHistoryObject>,
}

impl ConversationHistory {
    pub fn builder() -> ConversationHistoryBuilder {
        <ConversationHistoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryBuilder {
    conversation_id: Option<String>,
    entries: Option<Vec<ConversationHistoryEntriesItem>>,
    object: Option<ConversationHistoryObject>,
}

impl ConversationHistoryBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn entries(mut self, value: Vec<ConversationHistoryEntriesItem>) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn object(mut self, value: ConversationHistoryObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](ConversationHistoryBuilder::conversation_id)
    /// - [`entries`](ConversationHistoryBuilder::entries)
    pub fn build(self) -> Result<ConversationHistory, BuildError> {
        Ok(ConversationHistory {
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
            object: self.object,
        })
    }
}
