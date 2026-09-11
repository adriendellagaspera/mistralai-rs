#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRRequestConfidenceScoresGranularity {
    #[default]
    #[serde(rename = "word")]
    Word,
    #[serde(rename = "page")]
    Page,
}
