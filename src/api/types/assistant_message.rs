pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssistantMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<AssistantMessageContent>,
    /// Set this to `true` when adding an assistant message as prefix to condition the model response. The role of the prefix message is to force the model to start its answer by the content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AssistantMessageRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl AssistantMessage {
    pub fn builder() -> AssistantMessageBuilder {
        <AssistantMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssistantMessageBuilder {
    content: Option<AssistantMessageContent>,
    prefix: Option<bool>,
    role: Option<AssistantMessageRole>,
    tool_calls: Option<Vec<ToolCall>>,
}

impl AssistantMessageBuilder {
    pub fn content(mut self, value: AssistantMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn prefix(mut self, value: bool) -> Self {
        self.prefix = Some(value);
        self
    }

    pub fn role(mut self, value: AssistantMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssistantMessage`].
    pub fn build(self) -> Result<AssistantMessage, BuildError> {
        Ok(AssistantMessage {
            content: self.content,
            prefix: self.prefix,
            role: self.role,
            tool_calls: self.tool_calls,
        })
    }
}
