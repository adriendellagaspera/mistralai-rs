#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolReferenceChunkType {
    #[default]
    #[serde(rename = "tool_reference")]
    ToolReference,
}
