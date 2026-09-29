pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ThinkChunk {
    #[serde(default)]
    pub thinking: Vec<ThinkChunkThinkingItem>,
    /// Signature to replay some reasoning blocks across turns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Whether the thinking chunk is closed or not. Currently only used for prefixing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
}

impl ThinkChunk {
    pub fn builder() -> ThinkChunkBuilder {
        <ThinkChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThinkChunkBuilder {
    thinking: Option<Vec<ThinkChunkThinkingItem>>,
    signature: Option<String>,
    closed: Option<bool>,
}

impl ThinkChunkBuilder {
    pub fn thinking(mut self, value: Vec<ThinkChunkThinkingItem>) -> Self {
        self.thinking = Some(value);
        self
    }

    pub fn signature(mut self, value: impl Into<String>) -> Self {
        self.signature = Some(value.into());
        self
    }

    pub fn closed(mut self, value: bool) -> Self {
        self.closed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThinkChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thinking`](ThinkChunkBuilder::thinking)
    pub fn build(self) -> Result<ThinkChunk, BuildError> {
        Ok(ThinkChunk {
            thinking: self
                .thinking
                .ok_or_else(|| BuildError::missing_field("thinking"))?,
            signature: self.signature,
            closed: self.closed,
        })
    }
}
