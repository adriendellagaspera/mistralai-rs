///Typed error responses for `agents_api_v1_conversations_messages`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsMessagesApiError {
    Status422(HTTPValidationError),
}
