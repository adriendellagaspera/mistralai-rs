impl ConversationHistoryObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConversationHistory => "conversation.history",
        }
    }
}
