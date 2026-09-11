impl GetChatCompletionEventIdsInSchemaBuilder {
    /// Start a builder with every required wire field.
    pub fn new(search_params: FilterPayload) -> Self {
        Self {
            value: GetChatCompletionEventIdsInSchema::new(search_params),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "extra_fields", "` request field to a value."
    )]
    #[must_use]
    pub fn extra_fields(mut self, extra_fields: Vec<String>) -> Self {
        self.value.extra_fields = Some(Some(extra_fields));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "extra_fields", "` request field to JSON null."
    )]
    #[must_use]
    pub fn extra_fields_null(mut self) -> Self {
        self.value.extra_fields = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "extra_fields", "` request field.")]
    #[must_use]
    pub fn extra_fields_absent(mut self) -> Self {
        self.value.extra_fields = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> GetChatCompletionEventIdsInSchema {
        self.value
    }
}
