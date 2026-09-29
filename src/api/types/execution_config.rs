pub use crate::prelude::*;

/// Not typed since mcp config can changed / not stable
/// we allow all extra fields and this is a dict
/// TODO: once mcp is stable, we need to type this
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionConfig {
    #[serde(default)]
    pub r#type: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ExecutionConfig {
    pub fn builder() -> ExecutionConfigBuilder {
        <ExecutionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionConfigBuilder {
    r#type: Option<String>,
}

impl ExecutionConfigBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ExecutionConfigBuilder::r#type)
    pub fn build(self) -> Result<ExecutionConfig, BuildError> {
        Ok(ExecutionConfig {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
