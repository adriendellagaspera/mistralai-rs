///Attributes for workflow execution failed events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionFailedAttributes {
    pub failure: Failure,
    ///Unique identifier for the task within the workflow execution.
    pub task_id: String,
}
