impl RequestSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Playground => "playground",
            Self::AgentBuilderV1 => "agent_builder_v1",
        }
    }
}
