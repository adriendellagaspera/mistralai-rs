#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCall {
    pub arguments: FunctionCallArguments,
    pub name: String,
}
