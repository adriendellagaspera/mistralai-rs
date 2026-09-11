impl AgentHandoffEntryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AgentHandoff => "agent.handoff",
        }
    }
}
