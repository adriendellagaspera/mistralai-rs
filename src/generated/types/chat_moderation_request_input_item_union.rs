#[derive(Debug, Clone)]
pub enum ChatModerationRequestInputItemUnion {
    SystemMessage(SystemMessage),
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolMessage(ToolMessage),
}
