#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResponseErrorEventType {
    #[default]
    #[serde(rename = "conversation.response.error")]
    ConversationResponseError,
}
