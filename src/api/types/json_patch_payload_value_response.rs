pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum JsonPatchPayloadValueResponse {
    JsonPatchList(Vec<JsonPatch>),

    String(String),
}

impl JsonPatchPayloadValueResponse {
    pub fn is_json_patch_list(&self) -> bool {
        matches!(self, Self::JsonPatchList(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_json_patch_list(&self) -> Option<&Vec<JsonPatch>> {
        match self {
            Self::JsonPatchList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_json_patch_list(self) -> Option<Vec<JsonPatch>> {
        match self {
            Self::JsonPatchList(value) => Some(value),
            _ => None,
        }
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
}
