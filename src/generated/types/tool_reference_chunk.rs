#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolReferenceChunk {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub favicon: Option<Option<String>>,
    pub title: String,
    pub tool: ToolReferenceChunkTool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolReferenceChunkType>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub url: Option<Option<String>>,
}
