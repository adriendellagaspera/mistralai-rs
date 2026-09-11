impl ImageGenerationToolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ImageGeneration => "image_generation",
        }
    }
}
