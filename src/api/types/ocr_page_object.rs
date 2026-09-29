pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrPageObject {
    /// Paragraph-level bounding boxes for all content blocks in reading order (populated when include_blocks is True)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<OcrPageObjectBlocksItem>>,
    /// Confidence scores for the OCR page (populated when confidence_scores_granularity is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_scores: Option<OcrPageConfidenceScores>,
    /// The dimensions of the PDF Page's screenshot image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<OcrPageDimensions>,
    /// Footer of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// Header of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// List of all hyperlinks in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperlinks: Option<Vec<String>>,
    /// List of all extracted images in the page
    #[serde(default)]
    pub images: Vec<OcrImageObject>,
    /// The page index in a pdf document starting from 0
    #[serde(default)]
    pub index: i64,
    /// The markdown string response of the page
    #[serde(default)]
    pub markdown: String,
    /// List of all extracted tables in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<OcrTableObject>>,
}

impl OcrPageObject {
    pub fn builder() -> OcrPageObjectBuilder {
        <OcrPageObjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPageObjectBuilder {
    blocks: Option<Vec<OcrPageObjectBlocksItem>>,
    confidence_scores: Option<OcrPageConfidenceScores>,
    dimensions: Option<OcrPageDimensions>,
    footer: Option<String>,
    header: Option<String>,
    hyperlinks: Option<Vec<String>>,
    images: Option<Vec<OcrImageObject>>,
    index: Option<i64>,
    markdown: Option<String>,
    tables: Option<Vec<OcrTableObject>>,
}

impl OcrPageObjectBuilder {
    pub fn blocks(mut self, value: Vec<OcrPageObjectBlocksItem>) -> Self {
        self.blocks = Some(value);
        self
    }

    pub fn confidence_scores(mut self, value: OcrPageConfidenceScores) -> Self {
        self.confidence_scores = Some(value);
        self
    }

    pub fn dimensions(mut self, value: OcrPageDimensions) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn footer(mut self, value: impl Into<String>) -> Self {
        self.footer = Some(value.into());
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.header = Some(value.into());
        self
    }

    pub fn hyperlinks(mut self, value: Vec<String>) -> Self {
        self.hyperlinks = Some(value);
        self
    }

    pub fn images(mut self, value: Vec<OcrImageObject>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn markdown(mut self, value: impl Into<String>) -> Self {
        self.markdown = Some(value.into());
        self
    }

    pub fn tables(mut self, value: Vec<OcrTableObject>) -> Self {
        self.tables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrPageObject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`images`](OcrPageObjectBuilder::images)
    /// - [`index`](OcrPageObjectBuilder::index)
    /// - [`markdown`](OcrPageObjectBuilder::markdown)
    pub fn build(self) -> Result<OcrPageObject, BuildError> {
        Ok(OcrPageObject {
            blocks: self.blocks,
            confidence_scores: self.confidence_scores,
            dimensions: self.dimensions,
            footer: self.footer,
            header: self.header,
            hyperlinks: self.hyperlinks,
            images: self
                .images
                .ok_or_else(|| BuildError::missing_field("images"))?,
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            markdown: self
                .markdown
                .ok_or_else(|| BuildError::missing_field("markdown"))?,
            tables: self.tables,
        })
    }
}
