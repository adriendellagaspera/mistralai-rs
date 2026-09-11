#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolExecutionDoneEventType {
    #[default]
    #[serde(rename = "tool.execution.done")]
    ToolExecutionDone,
}
