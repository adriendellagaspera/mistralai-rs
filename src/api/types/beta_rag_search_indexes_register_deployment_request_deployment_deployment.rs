pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RegisterDeploymentRequestDeploymentDeployment {
    #[serde(rename = "vespa")]
    #[non_exhaustive]
    Vespa {
        #[serde(default)]
        indexes: Vec<RegisterDeploymentRequestVespaIndex>,
        #[serde(default)]
        query_url: String,
        #[serde(default)]
        vespa_version: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl RegisterDeploymentRequestDeploymentDeployment {
    pub fn vespa(
        indexes: Vec<RegisterDeploymentRequestVespaIndex>,
        query_url: String,
        vespa_version: String,
    ) -> Self {
        Self::Vespa {
            indexes,
            query_url,
            vespa_version,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
