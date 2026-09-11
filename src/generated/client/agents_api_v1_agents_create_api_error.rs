///Typed error responses for `agents_api_v1_agents_create`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1AgentsCreateApiError {
    Status422(HTTPValidationError),
}
