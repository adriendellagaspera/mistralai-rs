impl BatchJobInBuilder {
    /// Start a builder with every required wire field.
    pub fn new(endpoint: ApiEndpoint) -> Self {
        Self {
            value: BatchJobIn::new(endpoint),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "agent_id", "` request field to a value."
    )]
    #[must_use]
    pub fn agent_id(mut self, agent_id: String) -> Self {
        self.value.agent_id = Some(Some(agent_id));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "agent_id", "` request field to JSON null."
    )]
    #[must_use]
    pub fn agent_id_null(mut self) -> Self {
        self.value.agent_id = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "agent_id", "` request field.")]
    #[must_use]
    pub fn agent_id_absent(mut self) -> Self {
        self.value.agent_id = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "input_files", "` request field to a value."
    )]
    #[must_use]
    pub fn input_files(mut self, input_files: Vec<String>) -> Self {
        self.value.input_files = Some(Some(input_files));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "input_files", "` request field to JSON null."
    )]
    #[must_use]
    pub fn input_files_null(mut self) -> Self {
        self.value.input_files = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "input_files", "` request field.")]
    #[must_use]
    pub fn input_files_absent(mut self) -> Self {
        self.value.input_files = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to a value."
    )]
    #[must_use]
    pub fn metadata(mut self, metadata: BatchJobInMetadata) -> Self {
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
        "Set the optional nullable `", "model", "` request field to a value."
    )]
    #[must_use]
    pub fn model(mut self, model: String) -> Self {
        self.value.model = Some(Some(model));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "model", "` request field to JSON null."
    )]
    #[must_use]
    pub fn model_null(mut self) -> Self {
        self.value.model = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "model", "` request field.")]
    #[must_use]
    pub fn model_absent(mut self) -> Self {
        self.value.model = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "requests", "` request field to a value."
    )]
    #[must_use]
    pub fn requests(mut self, requests: Vec<BatchRequest>) -> Self {
        self.value.requests = Some(Some(requests));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "requests", "` request field to JSON null."
    )]
    #[must_use]
    pub fn requests_null(mut self) -> Self {
        self.value.requests = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "requests", "` request field.")]
    #[must_use]
    pub fn requests_absent(mut self) -> Self {
        self.value.requests = None;
        self
    }
    #[doc = concat!("Set the optional `", "timeout_hours", "` request field.")]
    #[must_use]
    pub fn timeout_hours(mut self, timeout_hours: i64) -> Self {
        self.value.timeout_hours = Some(timeout_hours);
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> BatchJobIn {
        self.value
    }
}
