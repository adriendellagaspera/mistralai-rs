///Confidence score for a token or word in OCR output.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRConfidenceScore {
    ///Confidence score (0-1)
    ///Constraint: minimum=0, maximum=1
    pub confidence: f64,
    ///Start index of the text in the page markdown string
    ///Constraint: minimum=0
    pub start_index: i64,
    ///The word or text segment
    pub text: String,
}
