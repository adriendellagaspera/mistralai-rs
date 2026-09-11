#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionTool {
    pub function: Function,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FunctionToolType>,
}
