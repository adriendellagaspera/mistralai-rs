pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssistantMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AssistantMessageRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<AssistantMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Set this to `true` when adding an assistant message as prefix to condition the model response. The role of the prefix message is to force the model to start its answer by the content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<bool>,
}

impl AssistantMessage {
    pub fn builder() -> AssistantMessageBuilder {
        <AssistantMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssistantMessageBuilder {
    role: Option<AssistantMessageRole>,
    content: Option<AssistantMessageContent>,
    tool_calls: Option<Vec<ToolCall>>,
    prefix: Option<bool>,
}

impl AssistantMessageBuilder {
    pub fn role(mut self, value: AssistantMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn content(mut self, value: AssistantMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn prefix(mut self, value: bool) -> Self {
        self.prefix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssistantMessage`].
    pub fn build(self) -> Result<AssistantMessage, BuildError> {
        Ok(AssistantMessage {
            role: self.role,
            content: self.content,
            tool_calls: self.tool_calls,
            prefix: self.prefix,
        })
    }
}
