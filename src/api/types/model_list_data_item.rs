pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ModelListDataItem {
    #[serde(rename = "base")]
    #[non_exhaustive]
    Base {
        #[serde(flatten)]
        data: BaseModelCard,
    },

    #[serde(rename = "fine-tuned")]
    #[non_exhaustive]
    FineTuned {
        #[serde(flatten)]
        data: FtModelCard,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ModelListDataItem {
    pub fn base(data: BaseModelCard) -> Self {
        Self::Base { data }
    }

    pub fn fine_tuned(data: FtModelCard) -> Self {
        Self::FineTuned { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
