pub use crate::prelude::*;

/// Per-block confidence scores, computed per-word from model logprobs.
///
/// All fields ``None`` when the block couldn't be scored.
/// Individual fields ``None`` when that signal is absent — e.g. an image-only block has
/// no caption, so content scores are ``None``.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrBlockConfidenceScores {
    /// Average confidence over the block's content (caption) tokens. None when the block has no textual content (e.g. image-only entry).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_content_confidence_score: Option<f64>,
    /// Minimum per-word content confidence in the block. None when the block has no textual content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_content_confidence_score: Option<f64>,
    /// Confidence in the block type (e.g. 'text', 'title', 'table'). None when the entry had no block type or the block type span could not be located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type_confidence_score: Option<f64>,
}

impl OcrBlockConfidenceScores {
    pub fn builder() -> OcrBlockConfidenceScoresBuilder {
        <OcrBlockConfidenceScoresBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrBlockConfidenceScoresBuilder {
    average_content_confidence_score: Option<f64>,
    minimum_content_confidence_score: Option<f64>,
    block_type_confidence_score: Option<f64>,
}

impl OcrBlockConfidenceScoresBuilder {
    pub fn average_content_confidence_score(mut self, value: f64) -> Self {
        self.average_content_confidence_score = Some(value);
        self
    }

    pub fn minimum_content_confidence_score(mut self, value: f64) -> Self {
        self.minimum_content_confidence_score = Some(value);
        self
    }

    pub fn block_type_confidence_score(mut self, value: f64) -> Self {
        self.block_type_confidence_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrBlockConfidenceScores`].
    pub fn build(self) -> Result<OcrBlockConfidenceScores, BuildError> {
        Ok(OcrBlockConfidenceScores {
            average_content_confidence_score: self.average_content_confidence_score,
            minimum_content_confidence_score: self.minimum_content_confidence_score,
            block_type_confidence_score: self.block_type_confidence_score,
        })
    }
}
