pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatClassificationRequestInputs {
    InstructRequest(InstructRequest),

    InstructRequestList(Vec<InstructRequest>),
}

impl ChatClassificationRequestInputs {
    pub fn is_instruct_request(&self) -> bool {
        matches!(self, Self::InstructRequest(_))
    }

    pub fn is_instruct_request_list(&self) -> bool {
        matches!(self, Self::InstructRequestList(_))
    }

    pub fn as_instruct_request(&self) -> Option<&InstructRequest> {
        match self {
            Self::InstructRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_instruct_request(self) -> Option<InstructRequest> {
        match self {
            Self::InstructRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_instruct_request_list(&self) -> Option<&Vec<InstructRequest>> {
        match self {
            Self::InstructRequestList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_instruct_request_list(self) -> Option<Vec<InstructRequest>> {
        match self {
            Self::InstructRequestList(value) => Some(value),
            _ => None,
        }
    }
}
