///Typed error responses for `agents_api_v1_conversations_append_stream`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsAppendStreamApiError {
    Status422(HTTPValidationError),
}
