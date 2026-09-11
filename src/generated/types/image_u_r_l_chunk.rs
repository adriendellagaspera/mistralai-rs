///{"type":"image_url","image_url":{"url":"data:image/png;base64,iVBORw0
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageURLChunk {
    pub image_url: ImageURLChunkImageUrl,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ImageURLChunkType>,
}
