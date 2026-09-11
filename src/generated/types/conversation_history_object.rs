#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConversationHistoryObject {
    #[default]
    #[serde(rename = "conversation.history")]
    ConversationHistory,
}
