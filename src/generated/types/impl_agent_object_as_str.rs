impl AgentObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Agent => "agent",
        }
    }
}
