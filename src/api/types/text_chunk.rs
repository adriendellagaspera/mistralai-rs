pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextChunk {
    #[serde(default)]
    pub text: String,
}

impl TextChunk {
    pub fn builder() -> TextChunkBuilder {
        <TextChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextChunkBuilder {
    text: Option<String>,
}

impl TextChunkBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TextChunkBuilder::text)
    pub fn build(self) -> Result<TextChunk, BuildError> {
        Ok(TextChunk {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
