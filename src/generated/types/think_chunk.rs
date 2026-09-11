#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThinkChunk {
    ///Whether the thinking chunk is closed or not. Currently only used for prefixing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    pub thinking: Vec<ThinkChunkThinkingItemUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ThinkChunkType>,
}
