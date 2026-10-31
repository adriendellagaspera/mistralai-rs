pub use crate::prelude::*;

/// Capability for client tasks operations (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClientTasksCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<TasksCancelCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<TasksListCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<ClientTasksRequestsCapability>,
}

impl ClientTasksCapability {
    pub fn builder() -> ClientTasksCapabilityBuilder {
        <ClientTasksCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientTasksCapabilityBuilder {
    cancel: Option<TasksCancelCapability>,
    list: Option<TasksListCapability>,
    requests: Option<ClientTasksRequestsCapability>,
}

impl ClientTasksCapabilityBuilder {
    pub fn cancel(mut self, value: TasksCancelCapability) -> Self {
        self.cancel = Some(value);
        self
    }

    pub fn list(mut self, value: TasksListCapability) -> Self {
        self.list = Some(value);
        self
    }

    pub fn requests(mut self, value: ClientTasksRequestsCapability) -> Self {
        self.requests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientTasksCapability`].
    pub fn build(self) -> Result<ClientTasksCapability, BuildError> {
        Ok(ClientTasksCapability {
            cancel: self.cancel,
            list: self.list,
            requests: self.requests,
        })
    }
}
