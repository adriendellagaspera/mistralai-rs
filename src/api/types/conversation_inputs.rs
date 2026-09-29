pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationInputs {
    String(String),

    InputEntries(InputEntries),
}

impl ConversationInputs {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_input_entries(&self) -> bool {
        matches!(self, Self::InputEntries(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_input_entries(&self) -> Option<&InputEntries> {
        match self {
            Self::InputEntries(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_input_entries(self) -> Option<InputEntries> {
        match self {
            Self::InputEntries(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ConversationInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::InputEntries(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
