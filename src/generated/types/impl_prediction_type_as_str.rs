impl PredictionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Content => "content",
        }
    }
}
