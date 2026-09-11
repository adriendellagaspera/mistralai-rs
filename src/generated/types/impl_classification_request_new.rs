impl ClassificationRequest {
    /// Construct this request with every required wire field.
    pub fn new(input: ClassificationRequestInput, model: String) -> Self {
        Self {
            input,
            model,
            metadata: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(
        input: ClassificationRequestInput,
        model: String,
    ) -> ClassificationRequestBuilder {
        ClassificationRequestBuilder::new(input, model)
    }
}
