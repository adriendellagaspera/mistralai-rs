impl MessageOutputEntryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MessageOutput => "message.output",
        }
    }
}
