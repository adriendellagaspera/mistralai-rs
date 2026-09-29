pub use crate::prelude::*;

/// Similar to the conversation history but only keep the messages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationMessages {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub messages: MessageEntries,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ConversationMessagesObject>,
}

impl ConversationMessages {
    pub fn builder() -> ConversationMessagesBuilder {
        <ConversationMessagesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationMessagesBuilder {
    conversation_id: Option<String>,
    messages: Option<MessageEntries>,
    object: Option<ConversationMessagesObject>,
}

impl ConversationMessagesBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn messages(mut self, value: MessageEntries) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn object(mut self, value: ConversationMessagesObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationMessages`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](ConversationMessagesBuilder::conversation_id)
    /// - [`messages`](ConversationMessagesBuilder::messages)
    pub fn build(self) -> Result<ConversationMessages, BuildError> {
        Ok(ConversationMessages {
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
            object: self.object,
        })
    }
}
