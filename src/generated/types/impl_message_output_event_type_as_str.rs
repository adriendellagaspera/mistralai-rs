impl MessageOutputEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MessageOutputDelta => "message.output.delta",
        }
    }
}
