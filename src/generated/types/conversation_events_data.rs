#[derive(Debug, Clone)]
pub enum ConversationEventsData {
    Started(ResponseStartedEvent),
    Done(ResponseDoneEvent),
    Error(ResponseErrorEvent),
    ToolExecutionStarted(ToolExecutionStartedEvent),
    ToolExecutionDelta(ToolExecutionDeltaEvent),
    ToolExecutionDone(ToolExecutionDoneEvent),
    MessageOutput(MessageOutputEvent),
    FunctionCall(FunctionCallEvent),
    AgentHandoffStarted(AgentHandoffStartedEvent),
    AgentHandoffDone(AgentHandoffDoneEvent),
}
