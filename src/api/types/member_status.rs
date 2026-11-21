pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemberStatus {
    Active,
    Deactivated,
    Blocked,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MemberStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Deactivated => serializer.serialize_str("deactivated"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MemberStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "deactivated" => Ok(Self::Deactivated),
            "blocked" => Ok(Self::Blocked),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MemberStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Deactivated => write!(f, "deactivated"),
            Self::Blocked => write!(f, "blocked"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
