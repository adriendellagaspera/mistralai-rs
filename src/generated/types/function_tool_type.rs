#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FunctionToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
