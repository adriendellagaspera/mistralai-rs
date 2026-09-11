#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResponseStartedEventType {
    #[default]
    #[serde(rename = "conversation.response.started")]
    ConversationResponseStarted,
}
