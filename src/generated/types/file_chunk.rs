#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileChunk {
    pub file_id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FileChunkType>,
}
