pub use crate::prelude::*;

/// Capability for tasks requests operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientTasksRequestsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elicitation: Option<TasksElicitationCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<TasksSamplingCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ClientTasksRequestsCapability {
    pub fn builder() -> ClientTasksRequestsCapabilityBuilder {
        <ClientTasksRequestsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientTasksRequestsCapabilityBuilder {
    elicitation: Option<TasksElicitationCapability>,
    sampling: Option<TasksSamplingCapability>,
}

impl ClientTasksRequestsCapabilityBuilder {
    pub fn elicitation(mut self, value: TasksElicitationCapability) -> Self {
        self.elicitation = Some(value);
        self
    }

    pub fn sampling(mut self, value: TasksSamplingCapability) -> Self {
        self.sampling = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientTasksRequestsCapability`].
    pub fn build(self) -> Result<ClientTasksRequestsCapability, BuildError> {
        Ok(ClientTasksRequestsCapability {
            elicitation: self.elicitation,
            sampling: self.sampling,
            extra: Default::default(),
        })
    }
}
