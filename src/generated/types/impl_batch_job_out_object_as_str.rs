impl BatchJobOutObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Batch => "batch",
        }
    }
}
