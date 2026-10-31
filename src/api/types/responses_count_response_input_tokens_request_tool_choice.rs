pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CountResponseInputTokensRequestToolChoice {
    CountResponseInputTokensRequestToolChoiceZero(CountResponseInputTokensRequestToolChoiceZero),

    ResponsesToolChoiceFunction(ResponsesToolChoiceFunction),
}

impl CountResponseInputTokensRequestToolChoice {
    pub fn is_count_response_input_tokens_request_tool_choice_zero(&self) -> bool {
        matches!(self, Self::CountResponseInputTokensRequestToolChoiceZero(_))
    }

    pub fn is_responses_tool_choice_function(&self) -> bool {
        matches!(self, Self::ResponsesToolChoiceFunction(_))
    }

    pub fn as_count_response_input_tokens_request_tool_choice_zero(
        &self,
    ) -> Option<&CountResponseInputTokensRequestToolChoiceZero> {
        match self {
            Self::CountResponseInputTokensRequestToolChoiceZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_count_response_input_tokens_request_tool_choice_zero(
        self,
    ) -> Option<CountResponseInputTokensRequestToolChoiceZero> {
        match self {
            Self::CountResponseInputTokensRequestToolChoiceZero(value) => Some(value),
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

impl fmt::Display for CountResponseInputTokensRequestToolChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountResponseInputTokensRequestToolChoiceZero(value) => write!(
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
