impl OCRFooterBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Footer => "footer",
        }
    }
}
