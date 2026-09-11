#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImageGenerationToolType {
    #[default]
    #[serde(rename = "image_generation")]
    ImageGeneration,
}
