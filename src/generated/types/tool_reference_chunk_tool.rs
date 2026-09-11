#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolReferenceChunkTool {
    BuiltInConnectors(BuiltInConnectors),
    String(String),
}
