#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRTableObject {
    ///Content of the table in the given format
    pub content: String,
    ///Format of the table
    pub format: OCRTableObjectFormat,
    ///Table ID for extracted table in a page
    pub id: String,
    ///Per-word confidence scores for the table content. Returned when confidence_scores_granularity is set to 'word'.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub word_confidence_scores: Option<Option<Vec<OCRConfidenceScore>>>,
}
