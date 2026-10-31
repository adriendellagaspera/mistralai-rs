pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicConnectorExecutionData {
    #[serde(default)]
    pub integrations: Vec<PublicExecutionConnector>,
    #[serde(default)]
    pub tools: Vec<ExecutionTool>,
}

impl PublicConnectorExecutionData {
    pub fn builder() -> PublicConnectorExecutionDataBuilder {
        <PublicConnectorExecutionDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicConnectorExecutionDataBuilder {
    integrations: Option<Vec<PublicExecutionConnector>>,
    tools: Option<Vec<ExecutionTool>>,
}

impl PublicConnectorExecutionDataBuilder {
    pub fn integrations(mut self, value: Vec<PublicExecutionConnector>) -> Self {
        self.integrations = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<ExecutionTool>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicConnectorExecutionData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`integrations`](PublicConnectorExecutionDataBuilder::integrations)
    /// - [`tools`](PublicConnectorExecutionDataBuilder::tools)
    pub fn build(self) -> Result<PublicConnectorExecutionData, BuildError> {
        Ok(PublicConnectorExecutionData {
            integrations: self
                .integrations
                .ok_or_else(|| BuildError::missing_field("integrations"))?,
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
