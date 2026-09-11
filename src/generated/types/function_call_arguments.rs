#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FunctionCallArguments {
    FunctionCallVariant(FunctionCallVariant),
    String(String),
}
