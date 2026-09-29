pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "role")]
#[non_exhaustive]
pub enum CompleteChatRequestMessagesItem {
    #[serde(rename = "assistant")]
    #[non_exhaustive]
    Assistant {
        #[serde(flatten)]
        data: AssistantMessage,
    },

    #[serde(rename = "system")]
    #[non_exhaustive]
    System {
        #[serde(flatten)]
        data: SystemMessage,
    },

    #[serde(rename = "tool")]
    #[non_exhaustive]
    Tool {
        #[serde(flatten)]
        data: ToolMessage,
    },

    #[serde(rename = "user")]
    #[non_exhaustive]
    User {
        #[serde(flatten)]
        data: UserMessage,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CompleteChatRequestMessagesItem {
    pub fn assistant(data: AssistantMessage) -> Self {
        Self::Assistant { data }
    }

    pub fn system(data: SystemMessage) -> Self {
        Self::System { data }
    }

    pub fn tool(data: ToolMessage) -> Self {
        Self::Tool { data }
    }

    pub fn user(data: UserMessage) -> Self {
        Self::User { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
