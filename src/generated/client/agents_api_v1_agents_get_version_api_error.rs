///Typed error responses for `agents_api_v1_agents_get_version`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1AgentsGetVersionApiError {
    Status422(HTTPValidationError),
}
