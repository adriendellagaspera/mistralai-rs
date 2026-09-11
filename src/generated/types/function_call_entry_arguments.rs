#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FunctionCallEntryArguments {
    FunctionCallEntryArgumentsVariant(FunctionCallEntryArgumentsVariant),
    String(String),
}
