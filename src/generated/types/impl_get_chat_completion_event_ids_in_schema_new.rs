impl GetChatCompletionEventIdsInSchema {
    /// Construct this request with every required wire field.
    pub fn new(search_params: FilterPayload) -> Self {
        Self {
            search_params,
            extra_fields: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(search_params: FilterPayload) -> GetChatCompletionEventIdsInSchemaBuilder {
        GetChatCompletionEventIdsInSchemaBuilder::new(search_params)
    }
}
