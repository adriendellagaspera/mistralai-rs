///Typed error responses for `agents_api_v1_conversations_restart_stream`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsRestartStreamApiError {
    Status422(HTTPValidationError),
}
