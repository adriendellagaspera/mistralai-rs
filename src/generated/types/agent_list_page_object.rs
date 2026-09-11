#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AgentListPageObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
