impl ConversationRestartRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(from_entry_id: String) -> Self {
        Self {
            value: ConversationRestartRequest::new(from_entry_id),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "agent_version", "` request field to a value."
    )]
    #[must_use]
    pub fn agent_version(
        mut self,
        agent_version: ConversationRestartRequestBaseAgentVersion,
    ) -> Self {
        self.value.agent_version = Some(Some(agent_version));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "agent_version", "` request field to JSON null."
    )]
    #[must_use]
    pub fn agent_version_null(mut self) -> Self {
        self.value.agent_version = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "agent_version", "` request field.")]
    #[must_use]
    pub fn agent_version_absent(mut self) -> Self {
        self.value.agent_version = None;
        self
    }
    #[doc = concat!("Set the optional `", "completion_args", "` request field.")]
    #[must_use]
    pub fn completion_args(mut self, completion_args: CompletionArgs) -> Self {
        self.value.completion_args = Some(completion_args);
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
    #[doc = concat!("Set the optional `", "handoff_execution", "` request field.")]
    #[must_use]
    pub fn handoff_execution(
        mut self,
        handoff_execution: ConversationRestartRequestBaseHandoffExecution,
    ) -> Self {
        self.value.handoff_execution = Some(handoff_execution);
        self
    }
    #[doc = concat!("Set the optional `", "inputs", "` request field.")]
    #[must_use]
    pub fn inputs(mut self, inputs: ConversationInputs) -> Self {
        self.value.inputs = Some(inputs);
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
    #[doc = concat!("Set the optional `", "store", "` request field.")]
    #[must_use]
    pub fn store(mut self, store: bool) -> Self {
        self.value.store = Some(store);
        self
    }
    #[doc = concat!("Set the optional `", "stream", "` request field.")]
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.value.stream = Some(stream);
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> ConversationRestartRequest {
        self.value
    }
}
