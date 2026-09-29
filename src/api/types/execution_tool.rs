pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_config: Option<ExecutionConfig>,
    #[serde(default)]
    pub integration_id: String,
    #[serde(default)]
    pub name: String,
}

impl ExecutionTool {
    pub fn builder() -> ExecutionToolBuilder {
        <ExecutionToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionToolBuilder {
    execution_config: Option<ExecutionConfig>,
    integration_id: Option<String>,
    name: Option<String>,
}

impl ExecutionToolBuilder {
    pub fn execution_config(mut self, value: ExecutionConfig) -> Self {
        self.execution_config = Some(value);
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`integration_id`](ExecutionToolBuilder::integration_id)
    /// - [`name`](ExecutionToolBuilder::name)
    pub fn build(self) -> Result<ExecutionTool, BuildError> {
        Ok(ExecutionTool {
            execution_config: self.execution_config,
            integration_id: self
                .integration_id
                .ok_or_else(|| BuildError::missing_field("integration_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
