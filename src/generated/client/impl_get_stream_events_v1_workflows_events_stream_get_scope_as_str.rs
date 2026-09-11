impl GetStreamEventsV1WorkflowsEventsStreamGetScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Activity => "activity",
            Self::Workflow => "workflow",
            Self::Value => "*",
        }
    }
}
