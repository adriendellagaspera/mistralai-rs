///Typed error responses for `fim_completion_v1_fim_completions_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum FimCompletionV1FimCompletionsPostApiError {
    Status422(HTTPValidationError),
}
