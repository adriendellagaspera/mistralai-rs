pub use crate::prelude::*;

/// Credentials-free projection of ExecutionEnv for the public /connectors/mistral response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicExecutionEnv {
    #[serde(default)]
    pub tools: Vec<Tool>,
    #[serde(default)]
    pub tool_execution_data: PublicConnectorExecutionData,
    #[serde(default)]
    pub errors: Vec<String>,
}

impl PublicExecutionEnv {
    pub fn builder() -> PublicExecutionEnvBuilder {
        <PublicExecutionEnvBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicExecutionEnvBuilder {
    tools: Option<Vec<Tool>>,
    tool_execution_data: Option<PublicConnectorExecutionData>,
    errors: Option<Vec<String>>,
}

impl PublicExecutionEnvBuilder {
    pub fn tools(mut self, value: Vec<Tool>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_execution_data(mut self, value: PublicConnectorExecutionData) -> Self {
        self.tool_execution_data = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicExecutionEnv`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tools`](PublicExecutionEnvBuilder::tools)
    /// - [`tool_execution_data`](PublicExecutionEnvBuilder::tool_execution_data)
    /// - [`errors`](PublicExecutionEnvBuilder::errors)
    pub fn build(self) -> Result<PublicExecutionEnv, BuildError> {
        Ok(PublicExecutionEnv {
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
            tool_execution_data: self
                .tool_execution_data
                .ok_or_else(|| BuildError::missing_field("tool_execution_data"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
        })
    }
}
