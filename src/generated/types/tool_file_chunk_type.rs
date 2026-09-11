#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolFileChunkType {
    #[default]
    #[serde(rename = "tool_file")]
    ToolFile,
}
