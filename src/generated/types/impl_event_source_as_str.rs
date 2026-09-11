impl EventSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Database => "DATABASE",
            Self::Live => "LIVE",
        }
    }
}
