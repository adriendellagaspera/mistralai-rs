pub use crate::prelude::*;

/// Confidence scores for an OCR page at various granularities.
///
/// Note on page-level stats:
/// - For 'page' and 'block' granularity: average/minimum are computed from per-token
/// exp(logprob). Neither ``word_confidence_scores`` nor ``token_scores`` is populated.
/// Per-block scores are attached to response blocks separately for 'block' granularity.
/// - For 'word' granularity: average/minimum are computed from per-word confidence, where
/// each word's confidence is exp(mean(token_logprobs)) — a geometric mean over the
/// word's subword tokens. ``word_confidence_scores`` is populated.
/// - For 'token' granularity (internal): average/minimum are computed from
/// ``token_scores``; ``token_scores`` is populated.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrPageConfidenceScores {
    /// Word-level confidence scores (populated only for 'word' granularity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_confidence_scores: Option<Vec<OcrConfidenceScore>>,
    /// Average confidence score for the page
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub average_page_confidence_score: f64,
    /// Minimum confidence score for the page
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub minimum_page_confidence_score: f64,
}

impl OcrPageConfidenceScores {
    pub fn builder() -> OcrPageConfidenceScoresBuilder {
        <OcrPageConfidenceScoresBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPageConfidenceScoresBuilder {
    word_confidence_scores: Option<Vec<OcrConfidenceScore>>,
    average_page_confidence_score: Option<f64>,
    minimum_page_confidence_score: Option<f64>,
}

impl OcrPageConfidenceScoresBuilder {
    pub fn word_confidence_scores(mut self, value: Vec<OcrConfidenceScore>) -> Self {
        self.word_confidence_scores = Some(value);
        self
    }

    pub fn average_page_confidence_score(mut self, value: f64) -> Self {
        self.average_page_confidence_score = Some(value);
        self
    }

    pub fn minimum_page_confidence_score(mut self, value: f64) -> Self {
        self.minimum_page_confidence_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrPageConfidenceScores`].
    /// This method will fail if any of the following fields are not set:
    /// - [`average_page_confidence_score`](OcrPageConfidenceScoresBuilder::average_page_confidence_score)
    /// - [`minimum_page_confidence_score`](OcrPageConfidenceScoresBuilder::minimum_page_confidence_score)
    pub fn build(self) -> Result<OcrPageConfidenceScores, BuildError> {
        Ok(OcrPageConfidenceScores {
            word_confidence_scores: self.word_confidence_scores,
            average_page_confidence_score: self
                .average_page_confidence_score
                .ok_or_else(|| BuildError::missing_field("average_page_confidence_score"))?,
            minimum_page_confidence_score: self
                .minimum_page_confidence_score
                .ok_or_else(|| BuildError::missing_field("minimum_page_confidence_score"))?,
        })
    }
}
