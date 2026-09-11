impl ConversationMessagesObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConversationMessages => "conversation.messages",
        }
    }
}
