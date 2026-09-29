pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatModerationRequestInput {
    ChatModerationRequestInputZeroItemList(Vec<ChatModerationRequestInputZeroItem>),

    ChatModerationRequestInputOneItemItemListList(Vec<Vec<ChatModerationRequestInputOneItemItem>>),
}

impl ChatModerationRequestInput {
    pub fn is_chat_moderation_request_input_zero_item_list(&self) -> bool {
        matches!(self, Self::ChatModerationRequestInputZeroItemList(_))
    }

    pub fn is_chat_moderation_request_input_one_item_item_list_list(&self) -> bool {
        matches!(self, Self::ChatModerationRequestInputOneItemItemListList(_))
    }

    pub fn as_chat_moderation_request_input_zero_item_list(
        &self,
    ) -> Option<&Vec<ChatModerationRequestInputZeroItem>> {
        match self {
            Self::ChatModerationRequestInputZeroItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_chat_moderation_request_input_zero_item_list(
        self,
    ) -> Option<Vec<ChatModerationRequestInputZeroItem>> {
        match self {
            Self::ChatModerationRequestInputZeroItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_chat_moderation_request_input_one_item_item_list_list(
        &self,
    ) -> Option<&Vec<Vec<ChatModerationRequestInputOneItemItem>>> {
        match self {
            Self::ChatModerationRequestInputOneItemItemListList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_chat_moderation_request_input_one_item_item_list_list(
        self,
    ) -> Option<Vec<Vec<ChatModerationRequestInputOneItemItem>>> {
        match self {
            Self::ChatModerationRequestInputOneItemItemListList(value) => Some(value),
            _ => None,
        }
    }
}
