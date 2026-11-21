pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "role")]
#[non_exhaustive]
pub enum SpeechRequestConversationHistoryItem {
    #[serde(rename = "assistant")]
    #[non_exhaustive]
    Assistant {
        #[serde(default)]
        audio: RawAudio,
        #[serde(default)]
        text: String,
    },

    #[serde(rename = "user")]
    #[non_exhaustive]
    User {
        #[serde(default)]
        audio: RawAudio,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SpeechRequestConversationHistoryItem {
    pub fn assistant(audio: RawAudio, text: String) -> Self {
        Self::Assistant { audio, text }
    }

    pub fn user(audio: RawAudio) -> Self {
        Self::User { audio }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
