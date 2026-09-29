pub use crate::prelude::*;

/// Query parameters for get_workflow_execution_trace_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowExecutionTraceEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_same_id_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_internal_events: Option<bool>,
}

impl GetWorkflowExecutionTraceEventsQueryRequest {
    pub fn builder() -> GetWorkflowExecutionTraceEventsQueryRequestBuilder {
        <GetWorkflowExecutionTraceEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowExecutionTraceEventsQueryRequestBuilder {
    merge_same_id_events: Option<bool>,
    include_internal_events: Option<bool>,
}

impl GetWorkflowExecutionTraceEventsQueryRequestBuilder {
    pub fn merge_same_id_events(mut self, value: bool) -> Self {
        self.merge_same_id_events = Some(value);
        self
    }

    pub fn include_internal_events(mut self, value: bool) -> Self {
        self.include_internal_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowExecutionTraceEventsQueryRequest`].
    pub fn build(self) -> Result<GetWorkflowExecutionTraceEventsQueryRequest, BuildError> {
        Ok(GetWorkflowExecutionTraceEventsQueryRequest {
            merge_same_id_events: self.merge_same_id_events,
            include_internal_events: self.include_internal_events,
        })
    }
}
