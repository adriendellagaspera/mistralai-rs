impl OCRListBlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::List => "list",
        }
    }
}
