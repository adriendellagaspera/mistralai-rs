#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolExecutionDeltaEventName {
    BuiltInConnectors(BuiltInConnectors),
    String(String),
}
