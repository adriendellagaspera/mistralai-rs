pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateResponseRequestInput {
    String(String),

    CreateResponseRequestInputOneItemList(Vec<CreateResponseRequestInputOneItem>),
}

impl CreateResponseRequestInput {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_create_response_request_input_one_item_list(&self) -> bool {
        matches!(self, Self::CreateResponseRequestInputOneItemList(_))
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

    pub fn as_create_response_request_input_one_item_list(
        &self,
    ) -> Option<&Vec<CreateResponseRequestInputOneItem>> {
        match self {
            Self::CreateResponseRequestInputOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_response_request_input_one_item_list(
        self,
    ) -> Option<Vec<CreateResponseRequestInputOneItem>> {
        match self {
            Self::CreateResponseRequestInputOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
