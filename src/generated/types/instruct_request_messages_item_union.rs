#[derive(Debug, Clone)]
pub enum InstructRequestMessagesItemUnion {
    SystemMessage(SystemMessage),
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolMessage(ToolMessage),
}
