impl OCRReferencesBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::References => "references",
        }
    }
}
