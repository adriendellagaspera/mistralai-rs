pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct InstructRequest {
    pub messages: Vec<InstructRequestMessagesItem>,
}

impl InstructRequest {
    pub fn builder() -> InstructRequestBuilder {
        <InstructRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstructRequestBuilder {
    messages: Option<Vec<InstructRequestMessagesItem>>,
}

impl InstructRequestBuilder {
    pub fn messages(mut self, value: Vec<InstructRequestMessagesItem>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InstructRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](InstructRequestBuilder::messages)
    pub fn build(self) -> Result<InstructRequest, BuildError> {
        Ok(InstructRequest {
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}
