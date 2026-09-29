pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JudgeConversationRequest {
    #[serde(default)]
    pub messages: Vec<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
}

impl JudgeConversationRequest {
    pub fn builder() -> JudgeConversationRequestBuilder {
        <JudgeConversationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeConversationRequestBuilder {
    messages: Option<Vec<HashMap<String, serde_json::Value>>>,
    properties: Option<HashMap<String, serde_json::Value>>,
}

impl JudgeConversationRequestBuilder {
    pub fn messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeConversationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](JudgeConversationRequestBuilder::messages)
    pub fn build(self) -> Result<JudgeConversationRequest, BuildError> {
        Ok(JudgeConversationRequest {
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
            properties: self.properties,
        })
    }
}
