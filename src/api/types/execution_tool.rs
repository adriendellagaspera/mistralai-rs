pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionTool {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub integration_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_config: Option<ExecutionConfig>,
}

impl ExecutionTool {
    pub fn builder() -> ExecutionToolBuilder {
        <ExecutionToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionToolBuilder {
    name: Option<String>,
    integration_id: Option<String>,
    execution_config: Option<ExecutionConfig>,
}

impl ExecutionToolBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    pub fn execution_config(mut self, value: ExecutionConfig) -> Self {
        self.execution_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ExecutionToolBuilder::name)
    /// - [`integration_id`](ExecutionToolBuilder::integration_id)
    pub fn build(self) -> Result<ExecutionTool, BuildError> {
        Ok(ExecutionTool {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            integration_id: self
                .integration_id
                .ok_or_else(|| BuildError::missing_field("integration_id"))?,
            execution_config: self.execution_config,
        })
    }
}
