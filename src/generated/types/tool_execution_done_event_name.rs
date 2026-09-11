#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolExecutionDoneEventName {
    BuiltInConnectors(BuiltInConnectors),
    String(String),
}
