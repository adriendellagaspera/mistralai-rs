pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CreateResponseResponseOutputItem {
    #[serde(rename = "message")]
    #[non_exhaustive]
    Message {
        #[serde(flatten)]
        data: ResponsesOutputMessage,
    },

    #[serde(rename = "function_call")]
    #[non_exhaustive]
    FunctionCall {
        #[serde(flatten)]
        data: ResponsesFunctionToolCall,
    },

    #[serde(rename = "reasoning")]
    #[non_exhaustive]
    Reasoning {
        #[serde(flatten)]
        data: ResponsesReasoningItem,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateResponseResponseOutputItem {
    pub fn message(data: ResponsesOutputMessage) -> Self {
        Self::Message { data }
    }

    pub fn function_call(data: ResponsesFunctionToolCall) -> Self {
        Self::FunctionCall { data }
    }

    pub fn reasoning(data: ResponsesReasoningItem) -> Self {
        Self::Reasoning { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
