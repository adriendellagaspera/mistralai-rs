#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowArchiveResponse {
    pub workflow: Workflow,
}
