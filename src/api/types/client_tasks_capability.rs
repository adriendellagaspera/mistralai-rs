pub use crate::prelude::*;

/// Capability for client tasks operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientTasksCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<TasksListCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<TasksCancelCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<ClientTasksRequestsCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ClientTasksCapability {
    pub fn builder() -> ClientTasksCapabilityBuilder {
        <ClientTasksCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientTasksCapabilityBuilder {
    list: Option<TasksListCapability>,
    cancel: Option<TasksCancelCapability>,
    requests: Option<ClientTasksRequestsCapability>,
}

impl ClientTasksCapabilityBuilder {
    pub fn list(mut self, value: TasksListCapability) -> Self {
        self.list = Some(value);
        self
    }

    pub fn cancel(mut self, value: TasksCancelCapability) -> Self {
        self.cancel = Some(value);
        self
    }

    pub fn requests(mut self, value: ClientTasksRequestsCapability) -> Self {
        self.requests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientTasksCapability`].
    pub fn build(self) -> Result<ClientTasksCapability, BuildError> {
        Ok(ClientTasksCapability {
            list: self.list,
            cancel: self.cancel,
            requests: self.requests,
            extra: Default::default(),
        })
    }
}
