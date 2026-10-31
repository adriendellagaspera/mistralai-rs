pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesFunctionCallOutputOutput {
    String(String),

    ResponsesFunctionCallOutputOutputOneItemList(Vec<ResponsesFunctionCallOutputOutputOneItem>),
}

impl ResponsesFunctionCallOutputOutput {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_responses_function_call_output_output_one_item_list(&self) -> bool {
        matches!(self, Self::ResponsesFunctionCallOutputOutputOneItemList(_))
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

    pub fn as_responses_function_call_output_output_one_item_list(
        &self,
    ) -> Option<&Vec<ResponsesFunctionCallOutputOutputOneItem>> {
        match self {
            Self::ResponsesFunctionCallOutputOutputOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_function_call_output_output_one_item_list(
        self,
    ) -> Option<Vec<ResponsesFunctionCallOutputOutputOneItem>> {
        match self {
            Self::ResponsesFunctionCallOutputOutputOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
