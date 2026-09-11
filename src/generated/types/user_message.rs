#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserMessage {
    pub content: Option<UserMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UserMessageRole>,
}
