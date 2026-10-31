pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateResponseRequestInputOneItem {
    ResponsesEasyInputMessage(ResponsesEasyInputMessage),

    ResponsesMessage(ResponsesMessage),

    ResponsesOutputMessage(ResponsesOutputMessage),

    ResponsesFunctionToolCall(ResponsesFunctionToolCall),

    ResponsesFunctionCallOutput(ResponsesFunctionCallOutput),

    ResponsesReasoningItem(ResponsesReasoningItem),
}

impl CreateResponseRequestInputOneItem {
    pub fn is_responses_easy_input_message(&self) -> bool {
        matches!(self, Self::ResponsesEasyInputMessage(_))
    }

    pub fn is_responses_message(&self) -> bool {
        matches!(self, Self::ResponsesMessage(_))
    }

    pub fn is_responses_output_message(&self) -> bool {
        matches!(self, Self::ResponsesOutputMessage(_))
    }

    pub fn is_responses_function_tool_call(&self) -> bool {
        matches!(self, Self::ResponsesFunctionToolCall(_))
    }

    pub fn is_responses_function_call_output(&self) -> bool {
        matches!(self, Self::ResponsesFunctionCallOutput(_))
    }

    pub fn is_responses_reasoning_item(&self) -> bool {
        matches!(self, Self::ResponsesReasoningItem(_))
    }

    pub fn as_responses_easy_input_message(&self) -> Option<&ResponsesEasyInputMessage> {
        match self {
            Self::ResponsesEasyInputMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_easy_input_message(self) -> Option<ResponsesEasyInputMessage> {
        match self {
            Self::ResponsesEasyInputMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_message(&self) -> Option<&ResponsesMessage> {
        match self {
            Self::ResponsesMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_message(self) -> Option<ResponsesMessage> {
        match self {
            Self::ResponsesMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_output_message(&self) -> Option<&ResponsesOutputMessage> {
        match self {
            Self::ResponsesOutputMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_output_message(self) -> Option<ResponsesOutputMessage> {
        match self {
            Self::ResponsesOutputMessage(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_function_tool_call(&self) -> Option<&ResponsesFunctionToolCall> {
        match self {
            Self::ResponsesFunctionToolCall(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_function_tool_call(self) -> Option<ResponsesFunctionToolCall> {
        match self {
            Self::ResponsesFunctionToolCall(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_function_call_output(&self) -> Option<&ResponsesFunctionCallOutput> {
        match self {
            Self::ResponsesFunctionCallOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_function_call_output(self) -> Option<ResponsesFunctionCallOutput> {
        match self {
            Self::ResponsesFunctionCallOutput(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_reasoning_item(&self) -> Option<&ResponsesReasoningItem> {
        match self {
            Self::ResponsesReasoningItem(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_reasoning_item(self) -> Option<ResponsesReasoningItem> {
        match self {
            Self::ResponsesReasoningItem(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateResponseRequestInputOneItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponsesEasyInputMessage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesMessage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesOutputMessage(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesFunctionToolCall(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesFunctionCallOutput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesReasoningItem(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
