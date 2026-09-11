#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolExecutionStartedEventType {
    #[default]
    #[serde(rename = "tool.execution.started")]
    ToolExecutionStarted,
}
