pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConversationHistoryEntriesItem {
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

    #[serde(rename = "function.result")]
    #[non_exhaustive]
    FunctionResult {
        #[serde(flatten)]
        data: FunctionResultEntry,
    },

    #[serde(rename = "function.call")]
    #[non_exhaustive]
    FunctionCall {
        #[serde(flatten)]
        data: FunctionCallEntry,
    },

    #[serde(rename = "tool.execution")]
    #[non_exhaustive]
    ToolExecution {
        #[serde(flatten)]
        data: ToolExecutionEntry,
    },

    #[serde(rename = "agent.handoff")]
    #[non_exhaustive]
    AgentHandoff {
        #[serde(flatten)]
        data: AgentHandoffEntry,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConversationHistoryEntriesItem {
    pub fn message_input(data: MessageInputEntry) -> Self {
        Self::MessageInput { data }
    }

    pub fn message_output(data: MessageOutputEntry) -> Self {
        Self::MessageOutput { data }
    }

    pub fn function_result(data: FunctionResultEntry) -> Self {
        Self::FunctionResult { data }
    }

    pub fn function_call(data: FunctionCallEntry) -> Self {
        Self::FunctionCall { data }
    }

    pub fn tool_execution(data: ToolExecutionEntry) -> Self {
        Self::ToolExecution { data }
    }

    pub fn agent_handoff(data: AgentHandoffEntry) -> Self {
        Self::AgentHandoff { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
