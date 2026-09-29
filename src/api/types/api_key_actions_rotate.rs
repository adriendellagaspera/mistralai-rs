pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum ApiKeyActionsRotate {
    #[serde(rename = "available")]
    #[non_exhaustive]
    Available {
        #[serde(flatten)]
        data: ActionAvailable,
    },

    #[serde(rename = "unavailable")]
    #[non_exhaustive]
    Unavailable {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<RotateUnavailableReason>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ApiKeyActionsRotate {
    pub fn available(data: ActionAvailable) -> Self {
        Self::Available { data }
    }

    pub fn unavailable() -> Self {
        Self::Unavailable { reason: None }
    }

    pub fn unavailable_with_reason(reason: RotateUnavailableReason) -> Self {
        Self::Unavailable {
            reason: Some(reason),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
