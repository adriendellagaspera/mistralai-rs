///Server side events sent when streaming a conversation response.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SSETypes {
    #[default]
    #[serde(rename = "conversation.response.started")]
    ConversationResponseStarted,
    #[serde(rename = "conversation.response.done")]
    ConversationResponseDone,
    #[serde(rename = "conversation.response.error")]
    ConversationResponseError,
    #[serde(rename = "message.output.delta")]
    MessageOutputDelta,
    #[serde(rename = "tool.execution.started")]
    ToolExecutionStarted,
    #[serde(rename = "tool.execution.delta")]
    ToolExecutionDelta,
    #[serde(rename = "tool.execution.done")]
    ToolExecutionDone,
    #[serde(rename = "agent.handoff.started")]
    AgentHandoffStarted,
    #[serde(rename = "agent.handoff.done")]
    AgentHandoffDone,
    #[serde(rename = "function.call.delta")]
    FunctionCallDelta,
}
