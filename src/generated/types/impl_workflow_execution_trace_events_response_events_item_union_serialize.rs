impl Serialize for WorkflowExecutionTraceEventsResponseEventsItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::WorkflowExecutionTraceEvent(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowExecutionProgressTraceEvent(value) => {
                serde::Serialize::serialize(value, serializer)
            }
        }
    }
}
