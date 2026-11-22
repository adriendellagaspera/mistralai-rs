pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetConversationsResponse {
    ModelConversation(ModelConversation),

    AgentConversation(AgentConversation),
}

impl GetConversationsResponse {
    pub fn is_model_conversation(&self) -> bool {
        matches!(self, Self::ModelConversation(_))
    }

    pub fn is_agent_conversation(&self) -> bool {
        matches!(self, Self::AgentConversation(_))
    }

    pub fn as_model_conversation(&self) -> Option<&ModelConversation> {
        match self {
            Self::ModelConversation(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_model_conversation(self) -> Option<ModelConversation> {
        match self {
            Self::ModelConversation(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_agent_conversation(&self) -> Option<&AgentConversation> {
        match self {
            Self::AgentConversation(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_agent_conversation(self) -> Option<AgentConversation> {
        match self {
            Self::AgentConversation(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetConversationsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModelConversation(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AgentConversation(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
