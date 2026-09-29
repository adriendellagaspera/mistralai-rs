pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectorDeactivateForConsumerV1ConnectorsRequestConsumerScope {
    User,
    Workspace,
    Organization,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectorDeactivateForConsumerV1ConnectorsRequestConsumerScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::Workspace => serializer.serialize_str("workspace"),
            Self::Organization => serializer.serialize_str("organization"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectorDeactivateForConsumerV1ConnectorsRequestConsumerScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "workspace" => Ok(Self::Workspace),
            "organization" => Ok(Self::Organization),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectorDeactivateForConsumerV1ConnectorsRequestConsumerScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Workspace => write!(f, "workspace"),
            Self::Organization => write!(f, "organization"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
