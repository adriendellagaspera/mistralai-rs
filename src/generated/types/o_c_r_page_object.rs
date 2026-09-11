#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRPageObject {
    ///Paragraph-level bounding boxes for all content blocks in reading order (populated when include_blocks is True)
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub blocks: Option<Option<Vec<OCRPageObjectBlocksItemUnion>>>,
    ///Confidence scores for the OCR page (populated when confidence_scores_granularity is set)
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub confidence_scores: Option<Option<OCRPageConfidenceScores>>,
    ///The dimensions of the PDF Page's screenshot image
    pub dimensions: Option<OCRPageDimensions>,
    ///Footer of the page
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub footer: Option<Option<String>>,
    ///Header of the page
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub header: Option<Option<String>>,
    ///List of all hyperlinks in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperlinks: Option<Vec<String>>,
    ///List of all extracted images in the page
    pub images: Vec<OCRImageObject>,
    ///The page index in a pdf document starting from 0
    ///Constraint: minimum=0
    pub index: i64,
    ///The markdown string response of the page
    pub markdown: String,
    ///List of all extracted tables in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<OCRTableObject>>,
}
