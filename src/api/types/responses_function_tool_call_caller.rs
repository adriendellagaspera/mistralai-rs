pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesFunctionToolCallCaller {
    ResponsesCallerDirect(ResponsesCallerDirect),

    ResponsesCallerProgram(ResponsesCallerProgram),
}

impl ResponsesFunctionToolCallCaller {
    pub fn is_responses_caller_direct(&self) -> bool {
        matches!(self, Self::ResponsesCallerDirect(_))
    }

    pub fn is_responses_caller_program(&self) -> bool {
        matches!(self, Self::ResponsesCallerProgram(_))
    }

    pub fn as_responses_caller_direct(&self) -> Option<&ResponsesCallerDirect> {
        match self {
            Self::ResponsesCallerDirect(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_caller_direct(self) -> Option<ResponsesCallerDirect> {
        match self {
            Self::ResponsesCallerDirect(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_caller_program(&self) -> Option<&ResponsesCallerProgram> {
        match self {
            Self::ResponsesCallerProgram(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_caller_program(self) -> Option<ResponsesCallerProgram> {
        match self {
            Self::ResponsesCallerProgram(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ResponsesFunctionToolCallCaller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponsesCallerDirect(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesCallerProgram(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
