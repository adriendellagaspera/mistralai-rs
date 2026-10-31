pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum DeploymentWorkerSpecResponseBackendSpec {
    #[serde(rename = "koyeb")]
    #[non_exhaustive]
    Koyeb {
        #[serde(flatten)]
        data: DeploymentMistralCloudBackendSpec,
    },

    #[serde(rename = "kubernetes")]
    #[non_exhaustive]
    Kubernetes {
        #[serde(skip_serializing_if = "Option::is_none")]
        entrypoint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        working_dir: Option<String>,
    },

    #[serde(rename = "mistral_cloud")]
    #[non_exhaustive]
    MistralCloud {
        #[serde(flatten)]
        data: DeploymentMistralCloudBackendSpec,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl DeploymentWorkerSpecResponseBackendSpec {
    pub fn koyeb(data: DeploymentMistralCloudBackendSpec) -> Self {
        Self::Koyeb { data }
    }

    pub fn kubernetes() -> Self {
        Self::Kubernetes {
            entrypoint: None,
            working_dir: None,
        }
    }

    pub fn mistral_cloud(data: DeploymentMistralCloudBackendSpec) -> Self {
        Self::MistralCloud { data }
    }

    pub fn kubernetes_with_entrypoint(entrypoint: String, working_dir: Option<String>) -> Self {
        Self::Kubernetes {
            entrypoint: Some(entrypoint),
            working_dir,
        }
    }

    pub fn kubernetes_with_working_dir(entrypoint: Option<String>, working_dir: String) -> Self {
        Self::Kubernetes {
            entrypoint,
            working_dir: Some(working_dir),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
