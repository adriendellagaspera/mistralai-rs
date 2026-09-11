impl ResponseStartedEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConversationResponseStarted => "conversation.response.started",
        }
    }
}
