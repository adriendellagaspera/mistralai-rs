///Typed error responses for `agents_api_v1_agents_delete_alias`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1AgentsDeleteAliasApiError {
    Status422(HTTPValidationError),
}
