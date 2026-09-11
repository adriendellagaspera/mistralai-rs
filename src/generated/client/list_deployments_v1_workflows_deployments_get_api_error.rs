///Typed error responses for `list_deployments_v1_workflows_deployments_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ListDeploymentsV1WorkflowsDeploymentsGetApiError {
    Status422(HTTPValidationError),
}
