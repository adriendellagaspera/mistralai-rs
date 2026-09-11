impl OCRTitleBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Title => "title",
        }
    }
}
