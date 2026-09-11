///Typed error responses for `agents_api_v1_conversations_history`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsHistoryApiError {
    Status422(HTTPValidationError),
}
