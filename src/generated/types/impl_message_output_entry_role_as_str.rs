impl MessageOutputEntryRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Assistant => "assistant",
        }
    }
}
