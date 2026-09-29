pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiPlan {
    Free,
    PayAsYouGo,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiPlan {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Free => serializer.serialize_str("FREE"),
            Self::PayAsYouGo => serializer.serialize_str("PAY_AS_YOU_GO"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiPlan {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "FREE" => Ok(Self::Free),
            "PAY_AS_YOU_GO" => Ok(Self::PayAsYouGo),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Free => write!(f, "FREE"),
            Self::PayAsYouGo => write!(f, "PAY_AS_YOU_GO"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
