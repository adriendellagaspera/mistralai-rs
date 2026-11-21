pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectorErrorCode {
    ConnectorsCountLimitReached,
    CredentialsRejected,
    ServerUnreachable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectorErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConnectorsCountLimitReached => {
                serializer.serialize_str("connectors_count_limit_reached")
            }
            Self::CredentialsRejected => serializer.serialize_str("credentials_rejected"),
            Self::ServerUnreachable => serializer.serialize_str("server_unreachable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectorErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "connectors_count_limit_reached" => Ok(Self::ConnectorsCountLimitReached),
            "credentials_rejected" => Ok(Self::CredentialsRejected),
            "server_unreachable" => Ok(Self::ServerUnreachable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectorErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectorsCountLimitReached => write!(f, "connectors_count_limit_reached"),
            Self::CredentialsRejected => write!(f, "credentials_rejected"),
            Self::ServerUnreachable => write!(f, "server_unreachable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
