pub use crate::prelude::*;

/// Query parameters for stream
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsExecutionsStreamQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<EventSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_id: Option<String>,
}

impl WorkflowsExecutionsStreamQueryRequest {
    pub fn builder() -> WorkflowsExecutionsStreamQueryRequestBuilder {
        <WorkflowsExecutionsStreamQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsExecutionsStreamQueryRequestBuilder {
    event_source: Option<EventSource>,
    last_event_id: Option<String>,
}

impl WorkflowsExecutionsStreamQueryRequestBuilder {
    pub fn event_source(mut self, value: EventSource) -> Self {
        self.event_source = Some(value);
        self
    }

    pub fn last_event_id(mut self, value: impl Into<String>) -> Self {
        self.last_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsExecutionsStreamQueryRequest`].
    pub fn build(self) -> Result<WorkflowsExecutionsStreamQueryRequest, BuildError> {
        Ok(WorkflowsExecutionsStreamQueryRequest {
            event_source: self.event_source,
            last_event_id: self.last_event_id,
        })
    }
}
