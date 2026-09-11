impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Event => "EVENT",
            Self::EventProgress => "EVENT_PROGRESS",
        }
    }
}
