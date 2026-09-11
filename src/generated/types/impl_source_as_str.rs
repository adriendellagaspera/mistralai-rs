impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Upload => "upload",
            Self::Repository => "repository",
            Self::Mistral => "mistral",
        }
    }
}
