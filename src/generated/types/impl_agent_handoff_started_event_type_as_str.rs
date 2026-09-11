impl AgentHandoffStartedEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AgentHandoffStarted => "agent.handoff.started",
        }
    }
}
