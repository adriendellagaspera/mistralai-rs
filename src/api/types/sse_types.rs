pub use crate::prelude::*;

/// Server side events sent when streaming a conversation response.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SseTypes {
    ConversationResponseStarted,
    ConversationResponseDone,
    ConversationResponseError,
    MessageOutputDelta,
    ToolExecutionStarted,
    ToolExecutionDelta,
    ToolExecutionDone,
    AgentHandoffStarted,
    AgentHandoffDone,
    FunctionCallDelta,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SseTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConversationResponseStarted => {
                serializer.serialize_str("conversation.response.started")
            }
            Self::ConversationResponseDone => {
                serializer.serialize_str("conversation.response.done")
            }
            Self::ConversationResponseError => {
                serializer.serialize_str("conversation.response.error")
            }
            Self::MessageOutputDelta => serializer.serialize_str("message.output.delta"),
            Self::ToolExecutionStarted => serializer.serialize_str("tool.execution.started"),
            Self::ToolExecutionDelta => serializer.serialize_str("tool.execution.delta"),
            Self::ToolExecutionDone => serializer.serialize_str("tool.execution.done"),
            Self::AgentHandoffStarted => serializer.serialize_str("agent.handoff.started"),
            Self::AgentHandoffDone => serializer.serialize_str("agent.handoff.done"),
            Self::FunctionCallDelta => serializer.serialize_str("function.call.delta"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SseTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversation.response.started" => Ok(Self::ConversationResponseStarted),
            "conversation.response.done" => Ok(Self::ConversationResponseDone),
            "conversation.response.error" => Ok(Self::ConversationResponseError),
            "message.output.delta" => Ok(Self::MessageOutputDelta),
            "tool.execution.started" => Ok(Self::ToolExecutionStarted),
            "tool.execution.delta" => Ok(Self::ToolExecutionDelta),
            "tool.execution.done" => Ok(Self::ToolExecutionDone),
            "agent.handoff.started" => Ok(Self::AgentHandoffStarted),
            "agent.handoff.done" => Ok(Self::AgentHandoffDone),
            "function.call.delta" => Ok(Self::FunctionCallDelta),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SseTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationResponseStarted => write!(f, "conversation.response.started"),
            Self::ConversationResponseDone => write!(f, "conversation.response.done"),
            Self::ConversationResponseError => write!(f, "conversation.response.error"),
            Self::MessageOutputDelta => write!(f, "message.output.delta"),
            Self::ToolExecutionStarted => write!(f, "tool.execution.started"),
            Self::ToolExecutionDelta => write!(f, "tool.execution.delta"),
            Self::ToolExecutionDone => write!(f, "tool.execution.done"),
            Self::AgentHandoffStarted => write!(f, "agent.handoff.started"),
            Self::AgentHandoffDone => write!(f, "agent.handoff.done"),
            Self::FunctionCallDelta => write!(f, "function.call.delta"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
