pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrTableObject {
    /// Table ID for extracted table in a page
    #[serde(default)]
    pub id: String,
    /// Content of the table in the given format
    #[serde(default)]
    pub content: String,
    /// Format of the table
    pub format: OcrTableObjectFormat,
    /// Per-word confidence scores for the table content. Returned when confidence_scores_granularity is set to 'word'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_confidence_scores: Option<Vec<OcrConfidenceScore>>,
}

impl OcrTableObject {
    pub fn builder() -> OcrTableObjectBuilder {
        <OcrTableObjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrTableObjectBuilder {
    id: Option<String>,
    content: Option<String>,
    format: Option<OcrTableObjectFormat>,
    word_confidence_scores: Option<Vec<OcrConfidenceScore>>,
}

impl OcrTableObjectBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn format(mut self, value: OcrTableObjectFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn word_confidence_scores(mut self, value: Vec<OcrConfidenceScore>) -> Self {
        self.word_confidence_scores = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrTableObject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OcrTableObjectBuilder::id)
    /// - [`content`](OcrTableObjectBuilder::content)
    /// - [`format`](OcrTableObjectBuilder::format)
    pub fn build(self) -> Result<OcrTableObject, BuildError> {
        Ok(OcrTableObject {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            word_confidence_scores: self.word_confidence_scores,
        })
    }
}
