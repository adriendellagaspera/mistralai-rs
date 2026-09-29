pub use crate::prelude::*;

/// Visibility options available to public API callers.
///
/// Excludes ``shared_global`` which is reserved for system-owned connectors.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublicResourceVisibility {
    SharedOrg,
    SharedWorkspace,
    Private,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublicResourceVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SharedOrg => serializer.serialize_str("shared_org"),
            Self::SharedWorkspace => serializer.serialize_str("shared_workspace"),
            Self::Private => serializer.serialize_str("private"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublicResourceVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "shared_org" => Ok(Self::SharedOrg),
            "shared_workspace" => Ok(Self::SharedWorkspace),
            "private" => Ok(Self::Private),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublicResourceVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SharedOrg => write!(f, "shared_org"),
            Self::SharedWorkspace => write!(f, "shared_workspace"),
            Self::Private => write!(f, "private"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
