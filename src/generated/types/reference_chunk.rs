#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReferenceChunk {
    pub reference_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ReferenceChunkType>,
}
