///Typed error responses for `get_workflow_registration_v1_workflows_registrations__workflow_registration_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetApiError {
    Status422(HTTPValidationError),
}
