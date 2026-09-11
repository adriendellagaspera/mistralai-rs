#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemMessage {
    pub content: SystemMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<SystemMessageRole>,
}
