#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MCPUIToolMetaVisibilityItem {
    #[default]
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "app")]
    App,
}
