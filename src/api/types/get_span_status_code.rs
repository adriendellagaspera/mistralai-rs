pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetSpanStatusCode {
    Error,
    Ok,
    Unset,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetSpanStatusCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Error => serializer.serialize_str("Error"),
            Self::Ok => serializer.serialize_str("Ok"),
            Self::Unset => serializer.serialize_str("Unset"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetSpanStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Error" => Ok(Self::Error),
            "Ok" => Ok(Self::Ok),
            "Unset" => Ok(Self::Unset),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetSpanStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "Error"),
            Self::Ok => write!(f, "Ok"),
            Self::Unset => write!(f, "Unset"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
