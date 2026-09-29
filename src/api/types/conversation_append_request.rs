pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationAppendRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<ConversationInputs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Whether to store the results into our servers or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_execution: Option<ConversationAppendRequestHandoffExecution>,
    /// Completion arguments that will be used to generate assistant responses. Can be overridden at each message request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_args: Option<CompletionArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_confirmations: Option<Vec<ToolCallConfirmation>>,
}

impl ConversationAppendRequest {
    pub fn builder() -> ConversationAppendRequestBuilder {
        <ConversationAppendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationAppendRequestBuilder {
    inputs: Option<ConversationInputs>,
    stream: Option<bool>,
    store: Option<bool>,
    handoff_execution: Option<ConversationAppendRequestHandoffExecution>,
    completion_args: Option<CompletionArgs>,
    tool_confirmations: Option<Vec<ToolCallConfirmation>>,
}

impl ConversationAppendRequestBuilder {
    pub fn inputs(mut self, value: ConversationInputs) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
        self
    }

    pub fn handoff_execution(mut self, value: ConversationAppendRequestHandoffExecution) -> Self {
        self.handoff_execution = Some(value);
        self
    }

    pub fn completion_args(mut self, value: CompletionArgs) -> Self {
        self.completion_args = Some(value);
        self
    }

    pub fn tool_confirmations(mut self, value: Vec<ToolCallConfirmation>) -> Self {
        self.tool_confirmations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationAppendRequest`].
    pub fn build(self) -> Result<ConversationAppendRequest, BuildError> {
        Ok(ConversationAppendRequest {
            inputs: self.inputs,
            stream: self.stream,
            store: self.store,
            handoff_execution: self.handoff_execution,
            completion_args: self.completion_args,
            tool_confirmations: self.tool_confirmations,
        })
    }
}
