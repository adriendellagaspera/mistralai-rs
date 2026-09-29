pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuthStatus {
    Valid,
    Invalid,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AuthStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Valid => serializer.serialize_str("valid"),
            Self::Invalid => serializer.serialize_str("invalid"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AuthStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "valid" => Ok(Self::Valid),
            "invalid" => Ok(Self::Invalid),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AuthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => write!(f, "valid"),
            Self::Invalid => write!(f, "invalid"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
