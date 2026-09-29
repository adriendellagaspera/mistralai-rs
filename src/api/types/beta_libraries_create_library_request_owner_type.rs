pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateLibraryRequestOwnerType {
    User,
    Workspace,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateLibraryRequestOwnerType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("User"),
            Self::Workspace => serializer.serialize_str("Workspace"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateLibraryRequestOwnerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "User" => Ok(Self::User),
            "Workspace" => Ok(Self::Workspace),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateLibraryRequestOwnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "User"),
            Self::Workspace => write!(f, "Workspace"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
