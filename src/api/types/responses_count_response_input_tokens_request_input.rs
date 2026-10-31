pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CountResponseInputTokensRequestInput {
    String(String),

    CountResponseInputTokensRequestInputOneItemList(
        Vec<CountResponseInputTokensRequestInputOneItem>,
    ),
}

impl CountResponseInputTokensRequestInput {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_count_response_input_tokens_request_input_one_item_list(&self) -> bool {
        matches!(
            self,
            Self::CountResponseInputTokensRequestInputOneItemList(_)
        )
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

    pub fn as_count_response_input_tokens_request_input_one_item_list(
        &self,
    ) -> Option<&Vec<CountResponseInputTokensRequestInputOneItem>> {
        match self {
            Self::CountResponseInputTokensRequestInputOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_count_response_input_tokens_request_input_one_item_list(
        self,
    ) -> Option<Vec<CountResponseInputTokensRequestInputOneItem>> {
        match self {
            Self::CountResponseInputTokensRequestInputOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
