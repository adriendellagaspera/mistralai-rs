pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OcrRequestPages {
    String(String),

    IntegerList(Vec<i64>),
}

impl OcrRequestPages {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_integer_list(&self) -> bool {
        matches!(self, Self::IntegerList(_))
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

    pub fn as_integer_list(&self) -> Option<&Vec<i64>> {
        match self {
            Self::IntegerList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_integer_list(self) -> Option<Vec<i64>> {
        match self {
            Self::IntegerList(value) => Some(value),
            _ => None,
        }
    }
}
