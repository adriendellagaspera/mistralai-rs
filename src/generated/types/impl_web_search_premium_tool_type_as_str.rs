impl WebSearchPremiumToolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WebSearchPremium => "web_search_premium",
        }
    }
}
