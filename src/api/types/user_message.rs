pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<UserMessageContent>,
}

impl UserMessage {
    pub fn builder() -> UserMessageBuilder {
        <UserMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserMessageBuilder {
    content: Option<UserMessageContent>,
}

impl UserMessageBuilder {
    pub fn content(mut self, value: UserMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserMessage`].
    pub fn build(self) -> Result<UserMessage, BuildError> {
        Ok(UserMessage {
            content: self.content,
        })
    }
}
