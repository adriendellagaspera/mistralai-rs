pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesInputText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    #[serde(default)]
    pub text: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesInputText {
    pub fn builder() -> ResponsesInputTextBuilder {
        <ResponsesInputTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesInputTextBuilder {
    prompt_cache_breakpoint: Option<ResponsesPromptCacheBreakpoint>,
    text: Option<String>,
}

impl ResponsesInputTextBuilder {
    pub fn prompt_cache_breakpoint(mut self, value: ResponsesPromptCacheBreakpoint) -> Self {
        self.prompt_cache_breakpoint = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesInputText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ResponsesInputTextBuilder::text)
    pub fn build(self) -> Result<ResponsesInputText, BuildError> {
        Ok(ResponsesInputText {
            prompt_cache_breakpoint: self.prompt_cache_breakpoint,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            extra: Default::default(),
        })
    }
}
