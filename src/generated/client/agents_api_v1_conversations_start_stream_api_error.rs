///Typed error responses for `agents_api_v1_conversations_start_stream`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsStartStreamApiError {
    Status422(HTTPValidationError),
}
