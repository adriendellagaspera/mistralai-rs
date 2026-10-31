pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ManagedIndexConfigEmbedding {
    #[serde(rename = "custom")]
    #[non_exhaustive]
    Custom {
        #[serde(default)]
        dimensions: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        distance_metric: Option<DistanceMetric>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dtype: Option<VectorDType>,
        #[serde(default)]
        name: String,
    },

    #[serde(rename = "mistral")]
    #[non_exhaustive]
    Mistral {
        #[serde(default)]
        dimensions: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        distance_metric: Option<DistanceMetric>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dtype: Option<VectorDType>,
        #[serde(default)]
        name: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ManagedIndexConfigEmbedding {
    pub fn custom(dimensions: i64, name: String) -> Self {
        Self::Custom {
            dimensions,
            distance_metric: None,
            dtype: None,
            name,
        }
    }

    pub fn mistral(dimensions: i64, name: String) -> Self {
        Self::Mistral {
            dimensions,
            distance_metric: None,
            dtype: None,
            name,
        }
    }

    pub fn custom_with_distance_metric(
        dimensions: i64,
        distance_metric: DistanceMetric,
        dtype: Option<VectorDType>,
        name: String,
    ) -> Self {
        Self::Custom {
            dimensions,
            distance_metric: Some(distance_metric),
            dtype,
            name,
        }
    }

    pub fn custom_with_dtype(
        dimensions: i64,
        distance_metric: Option<DistanceMetric>,
        dtype: VectorDType,
        name: String,
    ) -> Self {
        Self::Custom {
            dimensions,
            distance_metric,
            dtype: Some(dtype),
            name,
        }
    }

    pub fn mistral_with_distance_metric(
        dimensions: i64,
        distance_metric: DistanceMetric,
        dtype: Option<VectorDType>,
        name: String,
    ) -> Self {
        Self::Mistral {
            dimensions,
            distance_metric: Some(distance_metric),
            dtype,
            name,
        }
    }

    pub fn mistral_with_dtype(
        dimensions: i64,
        distance_metric: Option<DistanceMetric>,
        dtype: VectorDType,
        name: String,
    ) -> Self {
        Self::Mistral {
            dimensions,
            distance_metric,
            dtype: Some(dtype),
            name,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
