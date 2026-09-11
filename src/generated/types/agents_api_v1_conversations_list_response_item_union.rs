#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsListResponseItemUnion {
    ModelConversation(ModelConversation),
    AgentConversation(AgentConversation),
}
