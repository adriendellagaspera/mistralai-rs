///Typed error responses for `get_workflow_registrations_v1_workflows_registrations_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowRegistrationsV1WorkflowsRegistrationsGetApiError {
    Status422(HTTPValidationError),
}
