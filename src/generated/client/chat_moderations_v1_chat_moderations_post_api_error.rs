///Typed error responses for `chat_moderations_v1_chat_moderations_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ChatModerationsV1ChatModerationsPostApiError {
    Status422(HTTPValidationError),
}
