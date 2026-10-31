pub use crate::prelude::*;

/// Whether rows must match or must not match the supplied values.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdminUsageQueryFilterOperator {
    Is,
    IsNot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdminUsageQueryFilterOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Is => serializer.serialize_str("is"),
            Self::IsNot => serializer.serialize_str("is_not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdminUsageQueryFilterOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "is" => Ok(Self::Is),
            "is_not" => Ok(Self::IsNot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdminUsageQueryFilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Is => write!(f, "is"),
            Self::IsNot => write!(f, "is_not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
