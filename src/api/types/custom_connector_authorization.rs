pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CustomConnectorAuthorization {
    #[serde(rename = "api-key")]
    #[non_exhaustive]
    ApiKey {
        #[serde(default)]
        value: String,
    },

    #[serde(rename = "oauth2-token")]
    #[non_exhaustive]
    Oauth2Token {
        #[serde(default)]
        value: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CustomConnectorAuthorization {
    pub fn api_key(value: String) -> Self {
        Self::ApiKey { value }
    }

    pub fn oauth2token(value: String) -> Self {
        Self::Oauth2Token { value }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
