pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegistrySharingScope {
    SharingScopeUnspecified,
    Private,
    Workspace,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RegistrySharingScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SharingScopeUnspecified => serializer.serialize_str("sharing_scope_unspecified"),
            Self::Private => serializer.serialize_str("private"),
            Self::Workspace => serializer.serialize_str("workspace"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RegistrySharingScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sharing_scope_unspecified" => Ok(Self::SharingScopeUnspecified),
            "private" => Ok(Self::Private),
            "workspace" => Ok(Self::Workspace),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RegistrySharingScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SharingScopeUnspecified => write!(f, "sharing_scope_unspecified"),
            Self::Private => write!(f, "private"),
            Self::Workspace => write!(f, "workspace"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
