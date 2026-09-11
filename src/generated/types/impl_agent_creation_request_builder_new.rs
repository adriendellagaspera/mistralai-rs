impl AgentCreationRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(model: String, name: String) -> Self {
        Self {
            value: AgentCreationRequest::new(model, name),
        }
    }
    #[doc = concat!("Set the optional `", "completion_args", "` request field.")]
    #[must_use]
    pub fn completion_args(mut self, completion_args: CompletionArgs) -> Self {
        self.value.completion_args = Some(completion_args);
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
    #[doc = concat!(
        "Set the optional nullable `", "guardrails", "` request field to a value."
    )]
    #[must_use]
    pub fn guardrails(mut self, guardrails: Vec<GuardrailConfig>) -> Self {
        self.value.guardrails = Some(Some(guardrails));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "guardrails", "` request field to JSON null."
    )]
    #[must_use]
    pub fn guardrails_null(mut self) -> Self {
        self.value.guardrails = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "guardrails", "` request field.")]
    #[must_use]
    pub fn guardrails_absent(mut self) -> Self {
        self.value.guardrails = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "handoffs", "` request field to a value."
    )]
    #[must_use]
    pub fn handoffs(mut self, handoffs: Vec<String>) -> Self {
        self.value.handoffs = Some(Some(handoffs));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "handoffs", "` request field to JSON null."
    )]
    #[must_use]
    pub fn handoffs_null(mut self) -> Self {
        self.value.handoffs = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "handoffs", "` request field.")]
    #[must_use]
    pub fn handoffs_absent(mut self) -> Self {
        self.value.handoffs = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "instructions", "` request field to a value."
    )]
    #[must_use]
    pub fn instructions(mut self, instructions: String) -> Self {
        self.value.instructions = Some(Some(instructions));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "instructions", "` request field to JSON null."
    )]
    #[must_use]
    pub fn instructions_null(mut self) -> Self {
        self.value.instructions = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "instructions", "` request field.")]
    #[must_use]
    pub fn instructions_absent(mut self) -> Self {
        self.value.instructions = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to a value."
    )]
    #[must_use]
    pub fn metadata(mut self, metadata: MetadataDict) -> Self {
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
    #[doc = concat!("Set the optional `", "tools", "` request field.")]
    #[must_use]
    pub fn tools(mut self, tools: Vec<AgentCreationRequestToolsItemUnion>) -> Self {
        self.value.tools = Some(tools);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "version_message", "` request field to a value."
    )]
    #[must_use]
    pub fn version_message(mut self, version_message: String) -> Self {
        self.value.version_message = Some(Some(version_message));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "version_message", "` request field to JSON null."
    )]
    #[must_use]
    pub fn version_message_null(mut self) -> Self {
        self.value.version_message = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "version_message", "` request field."
    )]
    #[must_use]
    pub fn version_message_absent(mut self) -> Self {
        self.value.version_message = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> AgentCreationRequest {
        self.value
    }
}
