pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ToolMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ToolMessage {
    pub fn builder() -> ToolMessageBuilder {
        <ToolMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolMessageBuilder {
    content: Option<ToolMessageContent>,
    name: Option<String>,
    tool_call_id: Option<String>,
}

impl ToolMessageBuilder {
    pub fn content(mut self, value: ToolMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolMessage`].
    pub fn build(self) -> Result<ToolMessage, BuildError> {
        Ok(ToolMessage {
            content: self.content,
            name: self.name,
            tool_call_id: self.tool_call_id,
        })
    }
}
