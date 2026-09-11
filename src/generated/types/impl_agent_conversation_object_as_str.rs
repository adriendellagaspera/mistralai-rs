impl AgentConversationObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Conversation => "conversation",
        }
    }
}
