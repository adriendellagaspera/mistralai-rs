#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AgentHandoffStartedEventType {
    #[default]
    #[serde(rename = "agent.handoff.started")]
    AgentHandoffStarted,
}
