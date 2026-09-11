impl Serialize
    for ExecuteWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdExecutePostResponse
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::WorkflowExecutionResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::WorkflowExecutionSyncResponse(value) => {
                serde::Serialize::serialize(value, serializer)
            }
        }
    }
}
