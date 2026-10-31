pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateResponseRequestToolChoice {
    CreateResponseRequestToolChoiceZero(CreateResponseRequestToolChoiceZero),

    ResponsesToolChoiceFunction(ResponsesToolChoiceFunction),
}

impl CreateResponseRequestToolChoice {
    pub fn is_create_response_request_tool_choice_zero(&self) -> bool {
        matches!(self, Self::CreateResponseRequestToolChoiceZero(_))
    }

    pub fn is_responses_tool_choice_function(&self) -> bool {
        matches!(self, Self::ResponsesToolChoiceFunction(_))
    }

    pub fn as_create_response_request_tool_choice_zero(
        &self,
    ) -> Option<&CreateResponseRequestToolChoiceZero> {
        match self {
            Self::CreateResponseRequestToolChoiceZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_response_request_tool_choice_zero(
        self,
    ) -> Option<CreateResponseRequestToolChoiceZero> {
        match self {
            Self::CreateResponseRequestToolChoiceZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_tool_choice_function(&self) -> Option<&ResponsesToolChoiceFunction> {
        match self {
            Self::ResponsesToolChoiceFunction(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_tool_choice_function(self) -> Option<ResponsesToolChoiceFunction> {
        match self {
            Self::ResponsesToolChoiceFunction(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateResponseRequestToolChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateResponseRequestToolChoiceZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesToolChoiceFunction(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
