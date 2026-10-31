pub use crate::prelude::*;

/// Worker configuration for the Mistral Cloud backend.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentMistralCloudBackendSpec {
    /// Docker build context, as a path in the repo. Defaults to the repo root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_directory: Option<String>,
    /// Path to the Dockerfile, relative to 'build_directory'. Defaults to 'Dockerfile'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_path: Option<String>,
    /// Secrets to bind to the worker, as 'secret:workspace:<NAME>' references. Each is available during build and run: as a Dockerfile build arg (declare a matching 'ARG') and as an env var. On update, the field carries the complete set: send the full list to rebind, an empty list to unbind all, omit it to leave the bindings unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<DeploymentSecretBinding>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DeploymentMistralCloudBackendSpecType>,
}

impl DeploymentMistralCloudBackendSpec {
    pub fn builder() -> DeploymentMistralCloudBackendSpecBuilder {
        <DeploymentMistralCloudBackendSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentMistralCloudBackendSpecBuilder {
    build_directory: Option<String>,
    dockerfile_path: Option<String>,
    secrets: Option<Vec<DeploymentSecretBinding>>,
    r#type: Option<DeploymentMistralCloudBackendSpecType>,
}

impl DeploymentMistralCloudBackendSpecBuilder {
    pub fn build_directory(mut self, value: impl Into<String>) -> Self {
        self.build_directory = Some(value.into());
        self
    }

    pub fn dockerfile_path(mut self, value: impl Into<String>) -> Self {
        self.dockerfile_path = Some(value.into());
        self
    }

    pub fn secrets(mut self, value: Vec<DeploymentSecretBinding>) -> Self {
        self.secrets = Some(value);
        self
    }

    pub fn r#type(mut self, value: DeploymentMistralCloudBackendSpecType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentMistralCloudBackendSpec`].
    pub fn build(self) -> Result<DeploymentMistralCloudBackendSpec, BuildError> {
        Ok(DeploymentMistralCloudBackendSpec {
            build_directory: self.build_directory,
            dockerfile_path: self.dockerfile_path,
            secrets: self.secrets,
            r#type: self.r#type,
        })
    }
}
