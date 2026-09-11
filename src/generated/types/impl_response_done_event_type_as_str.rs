impl ResponseDoneEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConversationResponseDone => "conversation.response.done",
        }
    }
}
