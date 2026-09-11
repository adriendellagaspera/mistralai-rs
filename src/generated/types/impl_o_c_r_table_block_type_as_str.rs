impl OCRTableBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Table => "table",
        }
    }
}
