#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolFileChunk {
    pub file_id: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub file_name: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub file_type: Option<Option<String>>,
    pub tool: ToolFileChunkTool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolFileChunkType>,
}
