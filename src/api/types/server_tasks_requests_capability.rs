pub use crate::prelude::*;

/// Capability for tasks requests operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ServerTasksRequestsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<TasksToolsCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ServerTasksRequestsCapability {
    pub fn builder() -> ServerTasksRequestsCapabilityBuilder {
        <ServerTasksRequestsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServerTasksRequestsCapabilityBuilder {
    tools: Option<TasksToolsCapability>,
}

impl ServerTasksRequestsCapabilityBuilder {
    pub fn tools(mut self, value: TasksToolsCapability) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServerTasksRequestsCapability`].
    pub fn build(self) -> Result<ServerTasksRequestsCapability, BuildError> {
        Ok(ServerTasksRequestsCapability {
            tools: self.tools,
            extra: Default::default(),
        })
    }
}
