pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GetDeploymentSummariesResponseDeploymentDeployment {
    #[serde(rename = "vespa")]
    #[non_exhaustive]
    Vespa {
        #[serde(default)]
        indexes: Vec<GetDeploymentSummariesResponseVespaIndex>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl GetDeploymentSummariesResponseDeploymentDeployment {
    pub fn vespa(indexes: Vec<GetDeploymentSummariesResponseVespaIndex>) -> Self {
        Self::Vespa { indexes }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
