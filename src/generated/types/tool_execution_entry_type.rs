#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolExecutionEntryType {
    #[default]
    #[serde(rename = "tool.execution")]
    ToolExecution,
}
