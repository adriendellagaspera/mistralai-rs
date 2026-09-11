impl ImageContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Image => "image",
        }
    }
}
