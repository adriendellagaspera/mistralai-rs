pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetTraceStatusCode {
    Error,
    Unset,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetTraceStatusCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Error => serializer.serialize_str("Error"),
            Self::Unset => serializer.serialize_str("Unset"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetTraceStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Error" => Ok(Self::Error),
            "Unset" => Ok(Self::Unset),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetTraceStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "Error"),
            Self::Unset => write!(f, "Unset"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
