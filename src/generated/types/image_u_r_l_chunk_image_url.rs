#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ImageURLChunkImageUrl {
    ImageURL(ImageURL),
    String(String),
}
