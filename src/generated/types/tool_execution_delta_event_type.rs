#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolExecutionDeltaEventType {
    #[default]
    #[serde(rename = "tool.execution.delta")]
    ToolExecutionDelta,
}
