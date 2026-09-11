#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolFileChunkTool {
    BuiltInConnectors(BuiltInConnectors),
    String(String),
}
