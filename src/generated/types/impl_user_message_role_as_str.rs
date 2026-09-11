impl UserMessageRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
        }
    }
}
