#[derive(Debug, Clone)]
pub enum ChatCompletionRequestMessagesItemUnion {
    SystemMessage(SystemMessage),
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolMessage(ToolMessage),
}
