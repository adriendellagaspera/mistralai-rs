impl ImageURLChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ImageUrl => "image_url",
        }
    }
}
