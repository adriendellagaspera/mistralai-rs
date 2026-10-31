pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RangeLt {
    Integer(i64),

    Double(f64),

    DateTime(#[serde(with = "crate::core::flexible_datetime::offset")] DateTime<FixedOffset>),
}

impl RangeLt {
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_date_time(&self) -> bool {
        matches!(self, Self::DateTime(_))
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
}

impl fmt::Display for RangeLt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{}", value),
            Self::Double(value) => write!(f, "{}", value),
            Self::DateTime(value) => write!(f, "{}", value),
        }
    }
}
