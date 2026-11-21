pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordRetrieverOperator {
    Or,
    And,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for KeywordRetrieverOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Or => serializer.serialize_str("or"),
            Self::And => serializer.serialize_str("and"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for KeywordRetrieverOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "or" => Ok(Self::Or),
            "and" => Ok(Self::And),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for KeywordRetrieverOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Or => write!(f, "or"),
            Self::And => write!(f, "and"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
