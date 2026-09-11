impl ResponseErrorEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConversationResponseError => "conversation.response.error",
        }
    }
}
