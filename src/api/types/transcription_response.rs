pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TranscriptionResponse {
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegmentChunk>>,
    #[serde(default)]
    pub usage: UsageInfo,
}

impl TranscriptionResponse {
    pub fn builder() -> TranscriptionResponseBuilder {
        <TranscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptionResponseBuilder {
    model: Option<String>,
    text: Option<String>,
    language: Option<String>,
    segments: Option<Vec<TranscriptionSegmentChunk>>,
    usage: Option<UsageInfo>,
}

impl TranscriptionResponseBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn segments(mut self, value: Vec<TranscriptionSegmentChunk>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn usage(mut self, value: UsageInfo) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](TranscriptionResponseBuilder::model)
    /// - [`text`](TranscriptionResponseBuilder::text)
    /// - [`usage`](TranscriptionResponseBuilder::usage)
    pub fn build(self) -> Result<TranscriptionResponse, BuildError> {
        Ok(TranscriptionResponse {
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            language: self.language,
            segments: self.segments,
            usage: self
                .usage
                .ok_or_else(|| BuildError::missing_field("usage"))?,
        })
    }
}
