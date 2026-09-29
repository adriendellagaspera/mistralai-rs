pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserGroupTargetType {
    W,
    O,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UserGroupTargetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::W => serializer.serialize_str("W"),
            Self::O => serializer.serialize_str("O"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UserGroupTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "W" => Ok(Self::W),
            "O" => Ok(Self::O),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UserGroupTargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::W => write!(f, "W"),
            Self::O => write!(f, "O"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
