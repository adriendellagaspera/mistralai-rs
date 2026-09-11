impl ConversationResponseObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConversationResponse => "conversation.response",
        }
    }
}
