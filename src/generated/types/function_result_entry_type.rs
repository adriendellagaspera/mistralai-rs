#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FunctionResultEntryType {
    #[default]
    #[serde(rename = "function.result")]
    FunctionResult,
}
