impl FilePurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FineTune => "fine-tune",
            Self::Batch => "batch",
            Self::Ocr => "ocr",
        }
    }
}
