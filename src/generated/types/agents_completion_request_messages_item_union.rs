#[derive(Debug, Clone)]
pub enum AgentsCompletionRequestMessagesItemUnion {
    SystemMessage(SystemMessage),
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolMessage(ToolMessage),
}
