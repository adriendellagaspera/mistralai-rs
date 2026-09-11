impl EmbeddingRequest {
    /// Construct this request with every required wire field.
    pub fn new(input: EmbeddingRequestInput, model: String) -> Self {
        Self {
            input,
            model,
            encoding_format: None,
            metadata: None,
            output_dimension: None,
            output_dtype: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(input: EmbeddingRequestInput, model: String) -> EmbeddingRequestBuilder {
        EmbeddingRequestBuilder::new(input, model)
    }
}
