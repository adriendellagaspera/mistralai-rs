#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolTypes {
    #[default]
    #[serde(rename = "function")]
    Function,
}
