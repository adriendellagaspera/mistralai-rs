pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "purpose")]
#[non_exhaustive]
pub enum CreateClientSessionV1ClientSessionsPostSessionsRequest {
    #[serde(rename = "realtime")]
    #[non_exhaustive]
    Realtime {
        #[serde(default)]
        model: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        ttl_seconds: Option<i64>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateClientSessionV1ClientSessionsPostSessionsRequest {
    pub fn realtime(model: String) -> Self {
        Self::Realtime {
            model,
            ttl_seconds: None,
        }
    }

    pub fn realtime_with_ttl_seconds(model: String, ttl_seconds: i64) -> Self {
        Self::Realtime {
            model,
            ttl_seconds: Some(ttl_seconds),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
