pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TransferTargetDestination {
    #[serde(rename = "phone")]
    #[non_exhaustive]
    Phone {
        #[serde(default)]
        number: String,
    },

    #[serde(rename = "sip")]
    #[non_exhaustive]
    Sip {
        #[serde(default)]
        uri: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TransferTargetDestination {
    pub fn phone(number: String) -> Self {
        Self::Phone { number }
    }

    pub fn sip(uri: String) -> Self {
        Self::Sip { uri }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
