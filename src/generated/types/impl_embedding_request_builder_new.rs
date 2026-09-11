impl EmbeddingRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(input: EmbeddingRequestInput, model: String) -> Self {
        Self {
            value: EmbeddingRequest::new(input, model),
        }
    }
    #[doc = concat!("Set the optional `", "encoding_format", "` request field.")]
    #[must_use]
    pub fn encoding_format(mut self, encoding_format: EncodingFormat) -> Self {
        self.value.encoding_format = Some(encoding_format);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to a value."
    )]
    #[must_use]
    pub fn metadata(mut self, metadata: EmbeddingRequestMetadata) -> Self {
        self.value.metadata = Some(Some(metadata));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to JSON null."
    )]
    #[must_use]
    pub fn metadata_null(mut self) -> Self {
        self.value.metadata = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "metadata", "` request field.")]
    #[must_use]
    pub fn metadata_absent(mut self) -> Self {
        self.value.metadata = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "output_dimension", "` request field to a value."
    )]
    #[must_use]
    pub fn output_dimension(mut self, output_dimension: i64) -> Self {
        self.value.output_dimension = Some(Some(output_dimension));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "output_dimension",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn output_dimension_null(mut self) -> Self {
        self.value.output_dimension = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "output_dimension", "` request field."
    )]
    #[must_use]
    pub fn output_dimension_absent(mut self) -> Self {
        self.value.output_dimension = None;
        self
    }
    #[doc = concat!("Set the optional `", "output_dtype", "` request field.")]
    #[must_use]
    pub fn output_dtype(mut self, output_dtype: EmbeddingDtype) -> Self {
        self.value.output_dtype = Some(output_dtype);
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> EmbeddingRequest {
        self.value
    }
}
