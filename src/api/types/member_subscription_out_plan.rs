pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MemberSubscriptionOutPlan {
    ApiPlan(ApiPlan),

    ChatPlan(ChatPlan),

    CodePlan(CodePlan),
}

impl MemberSubscriptionOutPlan {
    pub fn is_api_plan(&self) -> bool {
        matches!(self, Self::ApiPlan(_))
    }

    pub fn is_chat_plan(&self) -> bool {
        matches!(self, Self::ChatPlan(_))
    }

    pub fn is_code_plan(&self) -> bool {
        matches!(self, Self::CodePlan(_))
    }

    pub fn as_api_plan(&self) -> Option<&ApiPlan> {
        match self {
            Self::ApiPlan(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_api_plan(self) -> Option<ApiPlan> {
        match self {
            Self::ApiPlan(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_chat_plan(&self) -> Option<&ChatPlan> {
        match self {
            Self::ChatPlan(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_chat_plan(self) -> Option<ChatPlan> {
        match self {
            Self::ChatPlan(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_code_plan(&self) -> Option<&CodePlan> {
        match self {
            Self::CodePlan(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_code_plan(self) -> Option<CodePlan> {
        match self {
            Self::CodePlan(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for MemberSubscriptionOutPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiPlan(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ChatPlan(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CodePlan(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
