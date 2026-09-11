#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolExecutionEntryName {
    BuiltInConnectors(BuiltInConnectors),
    String(String),
}
