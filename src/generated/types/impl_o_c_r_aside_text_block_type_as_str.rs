impl OCRAsideTextBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AsideText => "aside_text",
        }
    }
}
