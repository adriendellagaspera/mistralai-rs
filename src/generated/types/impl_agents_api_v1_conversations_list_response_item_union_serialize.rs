impl Serialize for AgentsApiV1ConversationsListResponseItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::ModelConversation(value) => serde::Serialize::serialize(value, serializer),
            Self::AgentConversation(value) => serde::Serialize::serialize(value, serializer),
        }
    }
}
