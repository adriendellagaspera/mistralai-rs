///Attributes for workflow task failed events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowTaskFailedAttributes {
    pub failure: Failure,
    ///Unique identifier for the task within the workflow execution.
    pub task_id: String,
}
