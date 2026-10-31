pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PipelineConfigSampling {
    #[serde(rename = "attribute_hash")]
    #[non_exhaustive]
    AttributeHash {
        #[serde(default)]
        attribute: String,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        percentage: f64,
    },

    #[serde(rename = "probabilistic")]
    #[non_exhaustive]
    Probabilistic {
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        percentage: f64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PipelineConfigSampling {
    pub fn attribute_hash(attribute: String, percentage: f64) -> Self {
        Self::AttributeHash {
            attribute,
            percentage,
        }
    }

    pub fn probabilistic(percentage: f64) -> Self {
        Self::Probabilistic { percentage }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
