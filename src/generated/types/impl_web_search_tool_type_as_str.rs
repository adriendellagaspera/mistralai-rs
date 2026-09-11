impl WebSearchToolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WebSearch => "web_search",
        }
    }
}
