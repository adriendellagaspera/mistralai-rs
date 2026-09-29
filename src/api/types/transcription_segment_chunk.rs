pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TranscriptionSegmentChunk {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start: f64,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TranscriptionSegmentChunkType>,
}

impl TranscriptionSegmentChunk {
    pub fn builder() -> TranscriptionSegmentChunkBuilder {
        <TranscriptionSegmentChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptionSegmentChunkBuilder {
    end: Option<f64>,
    score: Option<f64>,
    speaker_id: Option<String>,
    start: Option<f64>,
    text: Option<String>,
    r#type: Option<TranscriptionSegmentChunkType>,
}

impl TranscriptionSegmentChunkBuilder {
    pub fn end(mut self, value: f64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
        self
    }

    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TranscriptionSegmentChunkType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptionSegmentChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end`](TranscriptionSegmentChunkBuilder::end)
    /// - [`start`](TranscriptionSegmentChunkBuilder::start)
    /// - [`text`](TranscriptionSegmentChunkBuilder::text)
    pub fn build(self) -> Result<TranscriptionSegmentChunk, BuildError> {
        Ok(TranscriptionSegmentChunk {
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            score: self.score,
            speaker_id: self.speaker_id,
            start: self
                .start
                .ok_or_else(|| BuildError::missing_field("start"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type,
        })
    }
}
