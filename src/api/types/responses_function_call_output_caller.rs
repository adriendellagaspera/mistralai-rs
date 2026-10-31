pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesFunctionCallOutputCaller {
    ResponsesFunctionCallOutputCallerDirect(ResponsesFunctionCallOutputCallerDirect),

    ResponsesFunctionCallOutputCallerProgram(ResponsesFunctionCallOutputCallerProgram),
}

impl ResponsesFunctionCallOutputCaller {
    pub fn is_responses_function_call_output_caller_direct(&self) -> bool {
        matches!(self, Self::ResponsesFunctionCallOutputCallerDirect(_))
    }

    pub fn is_responses_function_call_output_caller_program(&self) -> bool {
        matches!(self, Self::ResponsesFunctionCallOutputCallerProgram(_))
    }

    pub fn as_responses_function_call_output_caller_direct(
        &self,
    ) -> Option<&ResponsesFunctionCallOutputCallerDirect> {
        match self {
            Self::ResponsesFunctionCallOutputCallerDirect(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_function_call_output_caller_direct(
        self,
    ) -> Option<ResponsesFunctionCallOutputCallerDirect> {
        match self {
            Self::ResponsesFunctionCallOutputCallerDirect(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_function_call_output_caller_program(
        &self,
    ) -> Option<&ResponsesFunctionCallOutputCallerProgram> {
        match self {
            Self::ResponsesFunctionCallOutputCallerProgram(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_function_call_output_caller_program(
        self,
    ) -> Option<ResponsesFunctionCallOutputCallerProgram> {
        match self {
            Self::ResponsesFunctionCallOutputCallerProgram(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ResponsesFunctionCallOutputCaller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponsesFunctionCallOutputCallerDirect(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesFunctionCallOutputCallerProgram(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
