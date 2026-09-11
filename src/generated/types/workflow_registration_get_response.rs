#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowRegistrationGetResponse {
    pub workflow_registration: WorkflowRegistrationWithWorkerStatus,
    pub workflow_version: WorkflowRegistrationWithWorkerStatus,
}
