///Typed error responses for `agents_api_v1_agents_list_pages`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1AgentsListPagesApiError {
    Status422(HTTPValidationError),
}
