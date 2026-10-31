pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OAuth2GrantType {
    AuthorizationCode,
    ClientCredentials,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OAuth2GrantType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AuthorizationCode => serializer.serialize_str("authorization_code"),
            Self::ClientCredentials => serializer.serialize_str("client_credentials"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OAuth2GrantType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "authorization_code" => Ok(Self::AuthorizationCode),
            "client_credentials" => Ok(Self::ClientCredentials),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OAuth2GrantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorizationCode => write!(f, "authorization_code"),
            Self::ClientCredentials => write!(f, "client_credentials"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
