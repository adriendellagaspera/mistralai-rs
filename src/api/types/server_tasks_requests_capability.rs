pub use crate::prelude::*;

/// Specifies which request types the server can augment with tasks (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ServerTasksRequestsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<TasksToolsCapability>,
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
        Ok(ServerTasksRequestsCapability { tools: self.tools })
    }
}
