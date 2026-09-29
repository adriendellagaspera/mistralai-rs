pub use crate::prelude::*;

/// The response after appending new entries to the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationResponse {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ConversationResponseObject>,
    #[serde(default)]
    pub outputs: Vec<ConversationResponseOutputsItem>,
    #[serde(default)]
    pub usage: ConversationUsageInfo,
}

impl ConversationResponse {
    pub fn builder() -> ConversationResponseBuilder {
        <ConversationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationResponseBuilder {
    conversation_id: Option<String>,
    guardrails: Option<Vec<HashMap<String, serde_json::Value>>>,
    object: Option<ConversationResponseObject>,
    outputs: Option<Vec<ConversationResponseOutputsItem>>,
    usage: Option<ConversationUsageInfo>,
}

impl ConversationResponseBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn guardrails(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.guardrails = Some(value);
        self
    }

    pub fn object(mut self, value: ConversationResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn outputs(mut self, value: Vec<ConversationResponseOutputsItem>) -> Self {
        self.outputs = Some(value);
        self
    }

    pub fn usage(mut self, value: ConversationUsageInfo) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](ConversationResponseBuilder::conversation_id)
    /// - [`outputs`](ConversationResponseBuilder::outputs)
    /// - [`usage`](ConversationResponseBuilder::usage)
    pub fn build(self) -> Result<ConversationResponse, BuildError> {
        Ok(ConversationResponse {
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            guardrails: self.guardrails,
            object: self.object,
            outputs: self
                .outputs
                .ok_or_else(|| BuildError::missing_field("outputs"))?,
            usage: self
                .usage
                .ok_or_else(|| BuildError::missing_field("usage"))?,
        })
    }
}
