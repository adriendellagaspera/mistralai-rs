///Typed error responses for `chat_classifications_v1_chat_classifications_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ChatClassificationsV1ChatClassificationsPostApiError {
    Status422(HTTPValidationError),
}
