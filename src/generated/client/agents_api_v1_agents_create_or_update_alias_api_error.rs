///Typed error responses for `agents_api_v1_agents_create_or_update_alias`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1AgentsCreateOrUpdateAliasApiError {
    Status422(HTTPValidationError),
}
