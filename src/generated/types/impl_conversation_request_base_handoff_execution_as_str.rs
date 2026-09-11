impl ConversationRequestBaseHandoffExecution {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Client => "client",
            Self::Server => "server",
        }
    }
}
