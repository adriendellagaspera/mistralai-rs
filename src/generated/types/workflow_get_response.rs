#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowGetResponse {
    pub workflow: WorkflowWithWorkerStatus,
}
