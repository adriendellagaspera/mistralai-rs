pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesEasyInputMessageContent {
    String(String),

    ResponsesEasyInputMessageContentOneItemList(Vec<ResponsesEasyInputMessageContentOneItem>),
}

impl ResponsesEasyInputMessageContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_responses_easy_input_message_content_one_item_list(&self) -> bool {
        matches!(self, Self::ResponsesEasyInputMessageContentOneItemList(_))
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

    pub fn as_responses_easy_input_message_content_one_item_list(
        &self,
    ) -> Option<&Vec<ResponsesEasyInputMessageContentOneItem>> {
        match self {
            Self::ResponsesEasyInputMessageContentOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_easy_input_message_content_one_item_list(
        self,
    ) -> Option<Vec<ResponsesEasyInputMessageContentOneItem>> {
        match self {
            Self::ResponsesEasyInputMessageContentOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
