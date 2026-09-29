pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConsumerType {
    User,
    Org,
    Workspace,
    System,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConsumerType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::Org => serializer.serialize_str("org"),
            Self::Workspace => serializer.serialize_str("workspace"),
            Self::System => serializer.serialize_str("system"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConsumerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "org" => Ok(Self::Org),
            "workspace" => Ok(Self::Workspace),
            "system" => Ok(Self::System),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConsumerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Org => write!(f, "org"),
            Self::Workspace => write!(f, "workspace"),
            Self::System => write!(f, "system"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
