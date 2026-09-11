impl OCRHeaderBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Header => "header",
        }
    }
}
