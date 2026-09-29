pub use crate::prelude::*;

/// Capability for server tasks operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ServerTasksCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<TasksCancelCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<TasksListCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<ServerTasksRequestsCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ServerTasksCapability {
    pub fn builder() -> ServerTasksCapabilityBuilder {
        <ServerTasksCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServerTasksCapabilityBuilder {
    cancel: Option<TasksCancelCapability>,
    list: Option<TasksListCapability>,
    requests: Option<ServerTasksRequestsCapability>,
}

impl ServerTasksCapabilityBuilder {
    pub fn cancel(mut self, value: TasksCancelCapability) -> Self {
        self.cancel = Some(value);
        self
    }

    pub fn list(mut self, value: TasksListCapability) -> Self {
        self.list = Some(value);
        self
    }

    pub fn requests(mut self, value: ServerTasksRequestsCapability) -> Self {
        self.requests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServerTasksCapability`].
    pub fn build(self) -> Result<ServerTasksCapability, BuildError> {
        Ok(ServerTasksCapability {
            cancel: self.cancel,
            list: self.list,
            requests: self.requests,
            extra: Default::default(),
        })
    }
}
