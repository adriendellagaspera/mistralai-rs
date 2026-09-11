#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseDoneEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ResponseDoneEventType>,
    pub usage: ConversationUsageInfo,
}
