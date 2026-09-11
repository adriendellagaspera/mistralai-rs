#[derive(Debug, Clone)]
pub enum InputEntriesItemUnion {
    MessageInputEntry(MessageInputEntry),
    MessageOutputEntry(MessageOutputEntry),
    FunctionResultEntry(FunctionResultEntry),
    FunctionCallEntry(FunctionCallEntry),
    ToolExecutionEntry(ToolExecutionEntry),
    AgentHandoffEntry(AgentHandoffEntry),
}
