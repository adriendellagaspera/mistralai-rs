pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum EvaluationResultSpec {
    #[serde(rename = "boolean")]
    #[non_exhaustive]
    Boolean {},

    #[serde(rename = "categorical")]
    #[non_exhaustive]
    Categorical {
        #[serde(default)]
        labels: Vec<String>,
    },

    #[serde(rename = "numeric")]
    #[non_exhaustive]
    Numeric {
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        max: f64,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        min: f64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl EvaluationResultSpec {
    pub fn boolean() -> Self {
        Self::Boolean {}
    }

    pub fn categorical(labels: Vec<String>) -> Self {
        Self::Categorical { labels }
    }

    pub fn numeric(max: f64, min: f64) -> Self {
        Self::Numeric { max, min }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
