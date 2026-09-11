///Typed error responses for `fim_completion_v1_fim_completions_post_stream`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum FimCompletionV1FimCompletionsPostStreamApiError {
    Status422(HTTPValidationError),
}
