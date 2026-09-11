#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolMessageRole {
    #[default]
    #[serde(rename = "tool")]
    Tool,
}
