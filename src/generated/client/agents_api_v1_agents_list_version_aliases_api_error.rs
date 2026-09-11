///Typed error responses for `agents_api_v1_agents_list_version_aliases`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1AgentsListVersionAliasesApiError {
    Status422(HTTPValidationError),
}
