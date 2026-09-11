#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImageURLChunkType {
    #[default]
    #[serde(rename = "image_url")]
    ImageUrl,
}
