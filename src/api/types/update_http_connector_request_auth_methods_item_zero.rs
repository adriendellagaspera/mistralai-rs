pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "method_type")]
#[non_exhaustive]
pub enum UpdateHttpConnectorRequestAuthMethodsItemZero {
    #[serde(rename = "bearer")]
    #[non_exhaustive]
    Bearer {
        #[serde(flatten)]
        data: BearerAuthMethod,
    },

    #[serde(rename = "none")]
    #[non_exhaustive]
    None {
        #[serde(flatten)]
        data: NoneAuthMethod,
    },

    #[serde(rename = "oauth2")]
    #[non_exhaustive]
    Oauth2 { value: OAuth2AuthMethod },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl UpdateHttpConnectorRequestAuthMethodsItemZero {
    pub fn bearer(data: BearerAuthMethod) -> Self {
        Self::Bearer { data }
    }

    pub fn none(data: NoneAuthMethod) -> Self {
        Self::None { data }
    }

    pub fn oauth2(value: OAuth2AuthMethod) -> Self {
        Self::Oauth2 { value }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
