///Typed error responses for `execute_workflow_registration_v1_workflows_registrations__workflow_registration_id__execute_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ExecuteWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdExecutePostApiError
{
    Status422(HTTPValidationError),
}
