pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ToolReferenceChunkTool {
    BuiltInConnectors(BuiltInConnectors),

    String(String),
}

impl ToolReferenceChunkTool {
    pub fn is_built_in_connectors(&self) -> bool {
        matches!(self, Self::BuiltInConnectors(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_built_in_connectors(&self) -> Option<&BuiltInConnectors> {
        match self {
            Self::BuiltInConnectors(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_built_in_connectors(self) -> Option<BuiltInConnectors> {
        match self {
            Self::BuiltInConnectors(value) => Some(value),
            _ => None,
        }
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
}

impl fmt::Display for ToolReferenceChunkTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuiltInConnectors(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
