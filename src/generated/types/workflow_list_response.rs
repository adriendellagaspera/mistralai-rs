#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowListResponse {
    pub next_cursor: Option<uuid::Uuid>,
    ///A list of workflows
    pub workflows: Vec<WorkflowBasicDefinition>,
}
