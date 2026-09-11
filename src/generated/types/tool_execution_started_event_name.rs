#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolExecutionStartedEventName {
    BuiltInConnectors(BuiltInConnectors),
    String(String),
}
