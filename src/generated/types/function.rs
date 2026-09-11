#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
    pub parameters: FunctionParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}
