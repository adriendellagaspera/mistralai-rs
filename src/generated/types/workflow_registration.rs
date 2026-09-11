#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowRegistration {
    ///Whether the workflow is compatible with chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_with_chat_assistant: Option<bool>,
    pub definition: WorkflowCodeDefinition,
    ///Unique identifier of the workflow registration
    pub id: uuid::Uuid,
    ///Project name of the workflow
    pub task_queue: String,
    ///Workflow of the workflow registration
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub workflow: Option<Option<Workflow>>,
    ///Workflow ID of the workflow
    pub workflow_id: uuid::Uuid,
}
