impl LibraryInBuilder {
    /// Start a builder with every required wire field.
    pub fn new(name: String) -> Self {
        Self {
            value: LibraryIn::new(name),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "chunk_size", "` request field to a value."
    )]
    #[must_use]
    pub fn chunk_size(mut self, chunk_size: i64) -> Self {
        self.value.chunk_size = Some(Some(chunk_size));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "chunk_size", "` request field to JSON null."
    )]
    #[must_use]
    pub fn chunk_size_null(mut self) -> Self {
        self.value.chunk_size = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "chunk_size", "` request field.")]
    #[must_use]
    pub fn chunk_size_absent(mut self) -> Self {
        self.value.chunk_size = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "description", "` request field to a value."
    )]
    #[must_use]
    pub fn description(mut self, description: String) -> Self {
        self.value.description = Some(Some(description));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "description", "` request field to JSON null."
    )]
    #[must_use]
    pub fn description_null(mut self) -> Self {
        self.value.description = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "description", "` request field.")]
    #[must_use]
    pub fn description_absent(mut self) -> Self {
        self.value.description = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> LibraryIn {
        self.value
    }
}
