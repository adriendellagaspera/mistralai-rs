pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum UpdateIndexMetricsV1RagDeploymentsDeploymentIdMetricsPutSearchIndexesRequestBody {
    #[serde(rename = "online")]
    #[non_exhaustive]
    Online {
        #[serde(default)]
        document_count: i64,
        #[serde(default)]
        index_metrics: Vec<UpdateMetricsRequestIndexMetrics>,
    },

    #[serde(rename = "offline")]
    #[non_exhaustive]
    Offline {
        #[serde(skip_serializing_if = "Option::is_none")]
        clear_metrics: Option<bool>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl UpdateIndexMetricsV1RagDeploymentsDeploymentIdMetricsPutSearchIndexesRequestBody {
    pub fn online(
        document_count: i64,
        index_metrics: Vec<UpdateMetricsRequestIndexMetrics>,
    ) -> Self {
        Self::Online {
            document_count,
            index_metrics,
        }
    }

    pub fn offline() -> Self {
        Self::Offline {
            clear_metrics: None,
        }
    }

    pub fn offline_with_clear_metrics(clear_metrics: bool) -> Self {
        Self::Offline {
            clear_metrics: Some(clear_metrics),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
