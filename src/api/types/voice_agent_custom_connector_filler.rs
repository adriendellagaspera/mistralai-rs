pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum VoiceAgentCustomConnectorFiller {
    #[serde(rename = "generated")]
    #[non_exhaustive]
    Generated {
        #[serde(flatten)]
        data: GeneratedToolFiller,
    },

    #[serde(rename = "static")]
    #[non_exhaustive]
    Static {
        #[serde(flatten)]
        data: StaticToolFiller,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl VoiceAgentCustomConnectorFiller {
    pub fn generated(data: GeneratedToolFiller) -> Self {
        Self::Generated { data }
    }

    pub fn r#static(data: StaticToolFiller) -> Self {
        Self::Static { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
