impl ClassificationRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(input: ClassificationRequestInput, model: String) -> Self {
        Self {
            value: ClassificationRequest::new(input, model),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to a value."
    )]
    #[must_use]
    pub fn metadata(mut self, metadata: ClassificationRequestMetadata) -> Self {
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
    /// Finish building the request model.
    pub fn build(self) -> ClassificationRequest {
        self.value
    }
}
