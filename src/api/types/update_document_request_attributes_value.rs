pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdateDocumentRequestAttributesValue {
    Boolean(bool),

    String(String),

    Integer(i64),

    Double(f64),

    DateTime(#[serde(with = "crate::core::flexible_datetime::offset")] DateTime<FixedOffset>),

    StringList(Vec<String>),

    IntegerList(Vec<i64>),

    DoubleList(Vec<f64>),

    BooleanList(Vec<bool>),
}

impl UpdateDocumentRequestAttributesValue {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_date_time(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn is_integer_list(&self) -> bool {
        matches!(self, Self::IntegerList(_))
    }

    pub fn is_double_list(&self) -> bool {
        matches!(self, Self::DoubleList(_))
    }

    pub fn is_boolean_list(&self) -> bool {
        matches!(self, Self::BooleanList(_))
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value),
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

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_date_time(&self) -> Option<&DateTime<FixedOffset>> {
        match self {
            Self::DateTime(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_date_time(self) -> Option<DateTime<FixedOffset>> {
        match self {
            Self::DateTime(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
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

    pub fn as_double_list(&self) -> Option<&Vec<f64>> {
        match self {
            Self::DoubleList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double_list(self) -> Option<Vec<f64>> {
        match self {
            Self::DoubleList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_boolean_list(&self) -> Option<&Vec<bool>> {
        match self {
            Self::BooleanList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_boolean_list(self) -> Option<Vec<bool>> {
        match self {
            Self::BooleanList(value) => Some(value),
            _ => None,
        }
    }
}
