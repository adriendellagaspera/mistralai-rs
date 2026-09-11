#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowUnarchiveResponse {
    pub workflow: Workflow,
}
