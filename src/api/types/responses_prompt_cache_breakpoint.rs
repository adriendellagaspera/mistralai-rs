pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesPromptCacheBreakpoint {
    pub mode: ResponsesPromptCacheBreakpointMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<ResponsesPromptCacheBreakpointTtl>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesPromptCacheBreakpoint {
    pub fn builder() -> ResponsesPromptCacheBreakpointBuilder {
        <ResponsesPromptCacheBreakpointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesPromptCacheBreakpointBuilder {
    mode: Option<ResponsesPromptCacheBreakpointMode>,
    ttl: Option<ResponsesPromptCacheBreakpointTtl>,
}

impl ResponsesPromptCacheBreakpointBuilder {
    pub fn mode(mut self, value: ResponsesPromptCacheBreakpointMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn ttl(mut self, value: ResponsesPromptCacheBreakpointTtl) -> Self {
        self.ttl = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesPromptCacheBreakpoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mode`](ResponsesPromptCacheBreakpointBuilder::mode)
    pub fn build(self) -> Result<ResponsesPromptCacheBreakpoint, BuildError> {
        Ok(ResponsesPromptCacheBreakpoint {
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            ttl: self.ttl,
            extra: Default::default(),
        })
    }
}
