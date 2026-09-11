impl TimestampGranularity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Segment => "segment",
            Self::Word => "word",
        }
    }
}
