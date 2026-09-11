#[derive(Debug, Clone)]
pub enum ConversationHistoryEntriesItemUnion {
    MessageInputEntry(MessageInputEntry),
    MessageOutputEntry(MessageOutputEntry),
    FunctionResultEntry(FunctionResultEntry),
    FunctionCallEntry(FunctionCallEntry),
    ToolExecutionEntry(ToolExecutionEntry),
    AgentHandoffEntry(AgentHandoffEntry),
}
