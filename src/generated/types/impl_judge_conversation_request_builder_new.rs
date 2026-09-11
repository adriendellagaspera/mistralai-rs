impl JudgeConversationRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(messages: Vec<JudgeConversationRequestMessagesItem>) -> Self {
        Self {
            value: JudgeConversationRequest::new(messages),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "properties", "` request field to a value."
    )]
    #[must_use]
    pub fn properties(mut self, properties: JudgeConversationRequestProperties) -> Self {
        self.value.properties = Some(Some(properties));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "properties", "` request field to JSON null."
    )]
    #[must_use]
    pub fn properties_null(mut self) -> Self {
        self.value.properties = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "properties", "` request field.")]
    #[must_use]
    pub fn properties_absent(mut self) -> Self {
        self.value.properties = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> JudgeConversationRequest {
        self.value
    }
}
