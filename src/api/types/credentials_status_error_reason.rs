pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CredentialsStatusErrorReason {
    OauthExpired,
    OauthNearExpiry,
    EmptyCredentials,
    UnparsableCredentials,
    YouNeedToReconnect,
    OauthRefreshError,
    McpServerUnreachable,
    McpServerTimedOut,
    McpServerError,
    UnknownError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CredentialsStatusErrorReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OauthExpired => serializer.serialize_str("oauth expired"),
            Self::OauthNearExpiry => serializer.serialize_str("oauth near expiry"),
            Self::EmptyCredentials => serializer.serialize_str("empty credentials"),
            Self::UnparsableCredentials => serializer.serialize_str("unparsable credentials"),
            Self::YouNeedToReconnect => serializer.serialize_str("you need to reconnect"),
            Self::OauthRefreshError => serializer.serialize_str("oauth refresh error"),
            Self::McpServerUnreachable => serializer.serialize_str("MCP server unreachable"),
            Self::McpServerTimedOut => serializer.serialize_str("MCP server timed out"),
            Self::McpServerError => serializer.serialize_str("MCP server error"),
            Self::UnknownError => serializer.serialize_str("unknown error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CredentialsStatusErrorReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "oauth expired" => Ok(Self::OauthExpired),
            "oauth near expiry" => Ok(Self::OauthNearExpiry),
            "empty credentials" => Ok(Self::EmptyCredentials),
            "unparsable credentials" => Ok(Self::UnparsableCredentials),
            "you need to reconnect" => Ok(Self::YouNeedToReconnect),
            "oauth refresh error" => Ok(Self::OauthRefreshError),
            "MCP server unreachable" => Ok(Self::McpServerUnreachable),
            "MCP server timed out" => Ok(Self::McpServerTimedOut),
            "MCP server error" => Ok(Self::McpServerError),
            "unknown error" => Ok(Self::UnknownError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CredentialsStatusErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OauthExpired => write!(f, "oauth expired"),
            Self::OauthNearExpiry => write!(f, "oauth near expiry"),
            Self::EmptyCredentials => write!(f, "empty credentials"),
            Self::UnparsableCredentials => write!(f, "unparsable credentials"),
            Self::YouNeedToReconnect => write!(f, "you need to reconnect"),
            Self::OauthRefreshError => write!(f, "oauth refresh error"),
            Self::McpServerUnreachable => write!(f, "MCP server unreachable"),
            Self::McpServerTimedOut => write!(f, "MCP server timed out"),
            Self::McpServerError => write!(f, "MCP server error"),
            Self::UnknownError => write!(f, "unknown error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
