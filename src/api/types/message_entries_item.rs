pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum MessageEntriesItem {
    #[serde(rename = "message.input")]
    #[non_exhaustive]
    MessageInput {
        #[serde(flatten)]
        data: MessageInputEntry,
    },

    #[serde(rename = "message.output")]
    #[non_exhaustive]
    MessageOutput {
        #[serde(flatten)]
        data: MessageOutputEntry,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl MessageEntriesItem {
    pub fn message_input(data: MessageInputEntry) -> Self {
        Self::MessageInput { data }
    }

    pub fn message_output(data: MessageOutputEntry) -> Self {
        Self::MessageOutput { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
