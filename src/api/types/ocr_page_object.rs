pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrPageObject {
    /// The page index in a pdf document starting from 0
    #[serde(default)]
    pub index: i64,
    /// The markdown string response of the page
    #[serde(default)]
    pub markdown: String,
    /// List of all extracted images in the page
    #[serde(default)]
    pub images: Vec<OcrImageObject>,
    /// List of all extracted tables in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<OcrTableObject>>,
    /// List of all hyperlinks in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperlinks: Option<Vec<String>>,
    /// Header of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// Footer of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// The dimensions of the PDF Page's screenshot image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<OcrPageDimensions>,
    /// Confidence scores for the OCR page (populated when confidence_scores_granularity is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_scores: Option<OcrPageConfidenceScores>,
    /// Paragraph-level bounding boxes for all content blocks in reading order (populated when include_blocks is True)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<OcrPageObjectBlocksItem>>,
}

impl OcrPageObject {
    pub fn builder() -> OcrPageObjectBuilder {
        <OcrPageObjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPageObjectBuilder {
    index: Option<i64>,
    markdown: Option<String>,
    images: Option<Vec<OcrImageObject>>,
    tables: Option<Vec<OcrTableObject>>,
    hyperlinks: Option<Vec<String>>,
    header: Option<String>,
    footer: Option<String>,
    dimensions: Option<OcrPageDimensions>,
    confidence_scores: Option<OcrPageConfidenceScores>,
    blocks: Option<Vec<OcrPageObjectBlocksItem>>,
}

impl OcrPageObjectBuilder {
    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn markdown(mut self, value: impl Into<String>) -> Self {
        self.markdown = Some(value.into());
        self
    }

    pub fn images(mut self, value: Vec<OcrImageObject>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn tables(mut self, value: Vec<OcrTableObject>) -> Self {
        self.tables = Some(value);
        self
    }

    pub fn hyperlinks(mut self, value: Vec<String>) -> Self {
        self.hyperlinks = Some(value);
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.header = Some(value.into());
        self
    }

    pub fn footer(mut self, value: impl Into<String>) -> Self {
        self.footer = Some(value.into());
        self
    }

    pub fn dimensions(mut self, value: OcrPageDimensions) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn confidence_scores(mut self, value: OcrPageConfidenceScores) -> Self {
        self.confidence_scores = Some(value);
        self
    }

    pub fn blocks(mut self, value: Vec<OcrPageObjectBlocksItem>) -> Self {
        self.blocks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrPageObject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`index`](OcrPageObjectBuilder::index)
    /// - [`markdown`](OcrPageObjectBuilder::markdown)
    /// - [`images`](OcrPageObjectBuilder::images)
    pub fn build(self) -> Result<OcrPageObject, BuildError> {
        Ok(OcrPageObject {
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            markdown: self
                .markdown
                .ok_or_else(|| BuildError::missing_field("markdown"))?,
            images: self
                .images
                .ok_or_else(|| BuildError::missing_field("images"))?,
            tables: self.tables,
            hyperlinks: self.hyperlinks,
            header: self.header,
            footer: self.footer,
            dimensions: self.dimensions,
            confidence_scores: self.confidence_scores,
            blocks: self.blocks,
        })
    }
}
