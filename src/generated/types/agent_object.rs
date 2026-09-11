#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AgentObject {
    #[default]
    #[serde(rename = "agent")]
    Agent,
}
