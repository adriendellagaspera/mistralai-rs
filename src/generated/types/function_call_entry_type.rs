#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FunctionCallEntryType {
    #[default]
    #[serde(rename = "function.call")]
    FunctionCall,
}
