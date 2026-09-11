#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuiltInConnectors {
    #[default]
    #[serde(rename = "web_search")]
    WebSearch,
    #[serde(rename = "web_search_premium")]
    WebSearchPremium,
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "image_generation")]
    ImageGeneration,
    #[serde(rename = "document_library")]
    DocumentLibrary,
}
