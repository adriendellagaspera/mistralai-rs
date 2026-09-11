///Retrieve all entries in a conversation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversationHistory {
    pub conversation_id: String,
    pub entries: Vec<ConversationHistoryEntriesItemUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ConversationHistoryObject>,
}
