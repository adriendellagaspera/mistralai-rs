impl OCRCodeBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Code => "code",
        }
    }
}
