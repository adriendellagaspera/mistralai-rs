#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseErrorEvent {
    pub code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ResponseErrorEventType>,
}
