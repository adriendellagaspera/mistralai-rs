///Confidence scores for an OCR page at various granularities.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRPageConfidenceScores {
    ///Average confidence score for the page
    ///Constraint: minimum=0, maximum=1
    pub average_page_confidence_score: f64,
    ///Minimum confidence score for the page
    ///Constraint: minimum=0, maximum=1
    pub minimum_page_confidence_score: f64,
    ///Word-level confidence scores (populated only for 'word' granularity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_confidence_scores: Option<Vec<OCRConfidenceScore>>,
}
