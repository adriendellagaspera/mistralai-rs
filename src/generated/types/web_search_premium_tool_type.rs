#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WebSearchPremiumToolType {
    #[default]
    #[serde(rename = "web_search_premium")]
    WebSearchPremium,
}
