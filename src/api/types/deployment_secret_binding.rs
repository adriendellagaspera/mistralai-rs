pub use crate::prelude::*;

/// Binds one env var to one platform secret; the value is resolved at boot, never stored.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentSecretBinding {
    /// Environment variable the worker process reads; the secret's name lives in 'reference'.
    #[serde(default)]
    pub env_var_name: String,
    /// Secret reference as 'secret:workspace:<NAME>'. Resolved at boot by the runtime-init container; the value never reaches the control plane.
    #[serde(default)]
    pub reference: String,
}

impl DeploymentSecretBinding {
    pub fn builder() -> DeploymentSecretBindingBuilder {
        <DeploymentSecretBindingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentSecretBindingBuilder {
    env_var_name: Option<String>,
    reference: Option<String>,
}

impl DeploymentSecretBindingBuilder {
    pub fn env_var_name(mut self, value: impl Into<String>) -> Self {
        self.env_var_name = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeploymentSecretBinding`].
    /// This method will fail if any of the following fields are not set:
    /// - [`env_var_name`](DeploymentSecretBindingBuilder::env_var_name)
    /// - [`reference`](DeploymentSecretBindingBuilder::reference)
    pub fn build(self) -> Result<DeploymentSecretBinding, BuildError> {
        Ok(DeploymentSecretBinding {
            env_var_name: self
                .env_var_name
                .ok_or_else(|| BuildError::missing_field("env_var_name"))?,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}
