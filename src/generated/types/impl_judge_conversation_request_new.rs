impl JudgeConversationRequest {
    /// Construct this request with every required wire field.
    pub fn new(messages: Vec<JudgeConversationRequestMessagesItem>) -> Self {
        Self {
            messages,
            properties: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(
        messages: Vec<JudgeConversationRequestMessagesItem>,
    ) -> JudgeConversationRequestBuilder {
        JudgeConversationRequestBuilder::new(messages)
    }
}
