#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowUpdateResponse {
    pub workflow: Workflow,
}
