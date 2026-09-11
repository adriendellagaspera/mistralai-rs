impl RealtimeTranscriptionSessionUpdateMessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SessionUpdate => "session.update",
        }
    }
}
