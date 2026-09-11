impl AgentHandoffDoneEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AgentHandoffDone => "agent.handoff.done",
        }
    }
}
