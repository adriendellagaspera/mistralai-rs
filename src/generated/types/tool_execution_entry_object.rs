#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolExecutionEntryObject {
    #[default]
    #[serde(rename = "entry")]
    Entry,
}
