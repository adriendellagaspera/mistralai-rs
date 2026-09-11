#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctx: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    pub loc: Vec<ValidationErrorLocItemUnion>,
    pub msg: String,
    pub r#type: String,
}
