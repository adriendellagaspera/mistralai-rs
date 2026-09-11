/**Emitted when a custom task completes successfully.

Contains the final result of the task execution.*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomTaskCompletedResponse {
    pub attributes: CustomTaskCompletedAttributesResponse,
    ///Unique identifier for this event instance.
    pub event_id: String,
    ///Unix timestamp in nanoseconds when the event was created.
    pub event_timestamp: i64,
    ///Event type discriminator.
    #[serde(default)]
    pub event_type: CustomTaskCompletedResponseEventType,
    ///Execution ID of the parent workflow that initiated this execution. If this is a root workflow, this field is not set.
    pub parent_workflow_exec_id: Option<String>,
    ///Execution ID of the root workflow that initiated this execution chain.
    pub root_workflow_exec_id: String,
    ///Execution ID of the workflow that emitted this event.
    pub workflow_exec_id: String,
    ///The registered name of the workflow that emitted this event.
    pub workflow_name: String,
    ///Run ID of the workflow execution. Changes on continue-as-new while workflow_exec_id stays the same.
    pub workflow_run_id: String,
}
