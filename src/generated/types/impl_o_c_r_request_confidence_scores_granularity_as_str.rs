impl OCRRequestConfidenceScoresGranularity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Word => "word",
            Self::Page => "page",
        }
    }
}
