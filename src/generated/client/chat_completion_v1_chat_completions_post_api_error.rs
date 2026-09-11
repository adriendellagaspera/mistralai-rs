///Typed error responses for `chat_completion_v1_chat_completions_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ChatCompletionV1ChatCompletionsPostApiError {
    Status422(HTTPValidationError),
}
