#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AgentHandoffEntryType {
    #[default]
    #[serde(rename = "agent.handoff")]
    AgentHandoff,
}
