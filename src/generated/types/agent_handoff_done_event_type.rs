#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AgentHandoffDoneEventType {
    #[default]
    #[serde(rename = "agent.handoff.done")]
    AgentHandoffDone,
}
