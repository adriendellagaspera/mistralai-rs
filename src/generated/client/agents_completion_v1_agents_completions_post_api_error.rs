///Typed error responses for `agents_completion_v1_agents_completions_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsCompletionV1AgentsCompletionsPostApiError {
    Status422(HTTPValidationError),
}
