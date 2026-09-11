impl MCPUIToolMetaVisibilityItem {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::App => "app",
        }
    }
}
