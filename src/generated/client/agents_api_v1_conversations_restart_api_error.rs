///Typed error responses for `agents_api_v1_conversations_restart`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsRestartApiError {
    Status422(HTTPValidationError),
}
