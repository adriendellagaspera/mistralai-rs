#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WebSearchToolType {
    #[default]
    #[serde(rename = "web_search")]
    WebSearch,
}
