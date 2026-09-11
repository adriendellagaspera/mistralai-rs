impl ConversationRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(inputs: ConversationInputs) -> Self {
        Self {
            value: ConversationRequest::new(inputs),
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
        "Set the optional nullable `", "agent_version", "` request field to a value."
    )]
    #[must_use]
    pub fn agent_version(mut self, agent_version: ConversationRequestBaseAgentVersion) -> Self {
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
    #[doc = concat!(
        "Set the optional nullable `", "completion_args", "` request field to a value."
    )]
    #[must_use]
    pub fn completion_args(mut self, completion_args: CompletionArgs) -> Self {
        self.value.completion_args = Some(Some(completion_args));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "completion_args", "` request field to JSON null."
    )]
    #[must_use]
    pub fn completion_args_null(mut self) -> Self {
        self.value.completion_args = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "completion_args", "` request field."
    )]
    #[must_use]
    pub fn completion_args_absent(mut self) -> Self {
        self.value.completion_args = None;
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
        "Set the optional nullable `", "handoff_execution", "` request field to a value."
    )]
    #[must_use]
    pub fn handoff_execution(
        mut self,
        handoff_execution: ConversationRequestBaseHandoffExecution,
    ) -> Self {
        self.value.handoff_execution = Some(Some(handoff_execution));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "handoff_execution",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn handoff_execution_null(mut self) -> Self {
        self.value.handoff_execution = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "handoff_execution", "` request field."
    )]
    #[must_use]
    pub fn handoff_execution_absent(mut self) -> Self {
        self.value.handoff_execution = None;
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
        "Set the optional nullable `", "name", "` request field to a value."
    )]
    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.value.name = Some(Some(name));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "name", "` request field to JSON null."
    )]
    #[must_use]
    pub fn name_null(mut self) -> Self {
        self.value.name = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "name", "` request field.")]
    #[must_use]
    pub fn name_absent(mut self) -> Self {
        self.value.name = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "store", "` request field to a value."
    )]
    #[must_use]
    pub fn store(mut self, store: bool) -> Self {
        self.value.store = Some(Some(store));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "store", "` request field to JSON null."
    )]
    #[must_use]
    pub fn store_null(mut self) -> Self {
        self.value.store = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "store", "` request field.")]
    #[must_use]
    pub fn store_absent(mut self) -> Self {
        self.value.store = None;
        self
    }
    #[doc = concat!("Set the optional `", "stream", "` request field.")]
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.value.stream = Some(stream);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "tools", "` request field to a value."
    )]
    #[must_use]
    pub fn tools(mut self, tools: Vec<ConversationRequestBaseToolsItemUnion>) -> Self {
        self.value.tools = Some(Some(tools));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "tools", "` request field to JSON null."
    )]
    #[must_use]
    pub fn tools_null(mut self) -> Self {
        self.value.tools = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "tools", "` request field.")]
    #[must_use]
    pub fn tools_absent(mut self) -> Self {
        self.value.tools = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> ConversationRequest {
        self.value
    }
}
