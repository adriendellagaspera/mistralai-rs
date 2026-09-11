#[derive(Debug, Clone)]
pub enum AgentsApiV1ConversationsGetResponse {
    ModelConversation(ModelConversation),
    AgentConversation(AgentConversation),
}
