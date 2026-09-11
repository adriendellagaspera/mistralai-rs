///Typed error responses for `agents_api_v1_conversations_list`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsListApiError {
    Status422(HTTPValidationError),
}
