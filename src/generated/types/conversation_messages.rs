///Similar to the conversation history but only keep the messages
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversationMessages {
    pub conversation_id: String,
    pub messages: MessageEntries,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ConversationMessagesObject>,
}
