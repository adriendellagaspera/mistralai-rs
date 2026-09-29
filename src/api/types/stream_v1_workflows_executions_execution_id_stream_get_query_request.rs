pub use crate::prelude::*;

/// Query parameters for stream_v1_workflows_executions__execution_id__stream_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<EventSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_id: Option<String>,
}

impl StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequest {
    pub fn builder() -> StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequestBuilder {
        <StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequestBuilder {
    event_source: Option<EventSource>,
    last_event_id: Option<String>,
}

impl StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequestBuilder {
    pub fn event_source(mut self, value: EventSource) -> Self {
        self.event_source = Some(value);
        self
    }

    pub fn last_event_id(mut self, value: impl Into<String>) -> Self {
        self.last_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequest, BuildError> {
        Ok(
            StreamV1WorkflowsExecutionsExecutionIdStreamGetQueryRequest {
                event_source: self.event_source,
                last_event_id: self.last_event_id,
            },
        )
    }
}
