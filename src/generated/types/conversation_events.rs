#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversationEvents {
    pub data: ConversationEventsData,
    pub event: SSETypes,
}
