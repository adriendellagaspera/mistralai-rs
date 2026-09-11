#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConversationMessagesObject {
    #[default]
    #[serde(rename = "conversation.messages")]
    ConversationMessages,
}
