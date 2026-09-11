#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    pub message: String,
}
