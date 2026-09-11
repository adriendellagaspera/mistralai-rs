impl BuiltInConnectors {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WebSearch => "web_search",
            Self::WebSearchPremium => "web_search_premium",
            Self::CodeInterpreter => "code_interpreter",
            Self::ImageGeneration => "image_generation",
            Self::DocumentLibrary => "document_library",
        }
    }
}
