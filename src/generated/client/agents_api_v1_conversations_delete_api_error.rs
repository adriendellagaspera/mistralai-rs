///Typed error responses for `agents_api_v1_conversations_delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsDeleteApiError {
    Status422(HTTPValidationError),
}
