pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemMessage {
    pub content: SystemMessageContent,
}

impl SystemMessage {
    pub fn builder() -> SystemMessageBuilder {
        <SystemMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemMessageBuilder {
    content: Option<SystemMessageContent>,
}

impl SystemMessageBuilder {
    pub fn content(mut self, value: SystemMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](SystemMessageBuilder::content)
    pub fn build(self) -> Result<SystemMessage, BuildError> {
        Ok(SystemMessage {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
