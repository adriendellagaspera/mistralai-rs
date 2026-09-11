///Typed error responses for `embeddings_v1_embeddings_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum EmbeddingsV1EmbeddingsPostApiError {
    Status422(HTTPValidationError),
}
