pub use crate::prelude::*;

/// Confidence score for a token or word in OCR output.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrConfidenceScore {
    /// The word or text segment
    #[serde(default)]
    pub text: String,
    /// Confidence score (0-1)
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
    /// Start index of the text in the page markdown string
    #[serde(default)]
    pub start_index: i64,
}

impl OcrConfidenceScore {
    pub fn builder() -> OcrConfidenceScoreBuilder {
        <OcrConfidenceScoreBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrConfidenceScoreBuilder {
    text: Option<String>,
    confidence: Option<f64>,
    start_index: Option<i64>,
}

impl OcrConfidenceScoreBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn start_index(mut self, value: i64) -> Self {
        self.start_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrConfidenceScore`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](OcrConfidenceScoreBuilder::text)
    /// - [`confidence`](OcrConfidenceScoreBuilder::confidence)
    /// - [`start_index`](OcrConfidenceScoreBuilder::start_index)
    pub fn build(self) -> Result<OcrConfidenceScore, BuildError> {
        Ok(OcrConfidenceScore {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
            start_index: self
                .start_index
                .ok_or_else(|| BuildError::missing_field("start_index"))?,
        })
    }
}
