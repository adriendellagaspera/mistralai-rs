impl SSETypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConversationResponseStarted => "conversation.response.started",
            Self::ConversationResponseDone => "conversation.response.done",
            Self::ConversationResponseError => "conversation.response.error",
            Self::MessageOutputDelta => "message.output.delta",
            Self::ToolExecutionStarted => "tool.execution.started",
            Self::ToolExecutionDelta => "tool.execution.delta",
            Self::ToolExecutionDone => "tool.execution.done",
            Self::AgentHandoffStarted => "agent.handoff.started",
            Self::AgentHandoffDone => "agent.handoff.done",
            Self::FunctionCallDelta => "function.call.delta",
        }
    }
}
