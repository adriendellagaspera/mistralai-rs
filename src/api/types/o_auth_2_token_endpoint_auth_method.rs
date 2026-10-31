pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OAuth2TokenEndpointAuthMethod {
    ClientSecretPost,
    ClientSecretBasic,
    PrivateKeyJwt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OAuth2TokenEndpointAuthMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ClientSecretPost => serializer.serialize_str("client_secret_post"),
            Self::ClientSecretBasic => serializer.serialize_str("client_secret_basic"),
            Self::PrivateKeyJwt => serializer.serialize_str("private_key_jwt"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OAuth2TokenEndpointAuthMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "client_secret_post" => Ok(Self::ClientSecretPost),
            "client_secret_basic" => Ok(Self::ClientSecretBasic),
            "private_key_jwt" => Ok(Self::PrivateKeyJwt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OAuth2TokenEndpointAuthMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClientSecretPost => write!(f, "client_secret_post"),
            Self::ClientSecretBasic => write!(f, "client_secret_basic"),
            Self::PrivateKeyJwt => write!(f, "private_key_jwt"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
