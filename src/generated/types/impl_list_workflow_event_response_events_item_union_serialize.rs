impl Serialize for ListWorkflowEventResponseEventsItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::WorkflowExecutionStartedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowExecutionCompletedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowExecutionFailedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowExecutionCanceledResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowExecutionContinuedAsNewResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowTaskTimedOutResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowTaskFailedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::CustomTaskStartedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::CustomTaskInProgressResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::CustomTaskCompletedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::CustomTaskFailedResponse(value) => serde::Serialize::serialize(value, serializer),
            Self::CustomTaskTimedOutResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::CustomTaskCanceledResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::ActivityTaskStartedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::ActivityTaskCompletedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::ActivityTaskRetryingResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::ActivityTaskFailedResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
        }
    }
}
