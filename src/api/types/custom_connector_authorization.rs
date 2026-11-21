pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CustomConnectorAuthorization {
    #[serde(rename = "api-key")]
    #[non_exhaustive]
    ApiKey {
        #[serde(flatten)]
        data: ApiKeyAuth,
    },

    #[serde(rename = "oauth2-token")]
    #[non_exhaustive]
    Oauth2Token {
        #[serde(flatten)]
        data: OAuth2TokenAuth,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CustomConnectorAuthorization {
    pub fn api_key(data: ApiKeyAuth) -> Self {
        Self::ApiKey { data }
    }

    pub fn oauth2token(data: OAuth2TokenAuth) -> Self {
        Self::Oauth2Token { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
