#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowRegistrationListResponse {
    pub next_cursor: Option<uuid::Uuid>,
    ///A list of workflow registrations
    pub workflow_registrations: Vec<WorkflowRegistration>,
    ///Deprecated: use workflow_registrations
    pub workflow_versions: Vec<WorkflowRegistration>,
}
