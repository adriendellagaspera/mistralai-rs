pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CompleteChatStreamRequestToolChoice {
    ToolChoice(ToolChoice),

    ToolChoiceEnum(ToolChoiceEnum),
}

impl CompleteChatStreamRequestToolChoice {
    pub fn is_tool_choice(&self) -> bool {
        matches!(self, Self::ToolChoice(_))
    }

    pub fn is_tool_choice_enum(&self) -> bool {
        matches!(self, Self::ToolChoiceEnum(_))
    }

    pub fn as_tool_choice(&self) -> Option<&ToolChoice> {
        match self {
            Self::ToolChoice(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tool_choice(self) -> Option<ToolChoice> {
        match self {
            Self::ToolChoice(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_tool_choice_enum(&self) -> Option<&ToolChoiceEnum> {
        match self {
            Self::ToolChoiceEnum(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tool_choice_enum(self) -> Option<ToolChoiceEnum> {
        match self {
            Self::ToolChoiceEnum(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CompleteChatStreamRequestToolChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolChoice(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ToolChoiceEnum(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
