#[derive(Debug, Clone)]
pub enum ConversationResponseOutputsItemUnion {
    MessageOutputEntry(MessageOutputEntry),
    ToolExecutionEntry(ToolExecutionEntry),
    FunctionCallEntry(FunctionCallEntry),
    AgentHandoffEntry(AgentHandoffEntry),
}
