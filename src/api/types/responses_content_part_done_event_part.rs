pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponsesContentPartDoneEventPart {
    #[serde(rename = "output_text")]
    #[non_exhaustive]
    OutputText {
        #[serde(flatten)]
        data: ResponsesOutputText,
    },

    #[serde(rename = "refusal")]
    #[non_exhaustive]
    Refusal {
        #[serde(flatten)]
        data: ResponsesOutputRefusal,
    },

    #[serde(rename = "reasoning_text")]
    #[non_exhaustive]
    ReasoningText {
        #[serde(flatten)]
        data: ResponsesPartReasoningText,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResponsesContentPartDoneEventPart {
    pub fn output_text(data: ResponsesOutputText) -> Self {
        Self::OutputText { data }
    }

    pub fn refusal(data: ResponsesOutputRefusal) -> Self {
        Self::Refusal { data }
    }

    pub fn reasoning_text(data: ResponsesPartReasoningText) -> Self {
        Self::ReasoningText { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
