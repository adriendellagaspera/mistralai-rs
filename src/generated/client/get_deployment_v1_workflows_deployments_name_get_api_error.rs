///Typed error responses for `get_deployment_v1_workflows_deployments__name__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetDeploymentV1WorkflowsDeploymentsNameGetApiError {
    Status422(HTTPValidationError),
}
