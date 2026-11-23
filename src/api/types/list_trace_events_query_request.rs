pub use crate::prelude::*;

/// Query parameters for list_trace_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTraceEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_same_id_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_internal_events: Option<bool>,
}

impl ListTraceEventsQueryRequest {
    pub fn builder() -> ListTraceEventsQueryRequestBuilder {
        <ListTraceEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTraceEventsQueryRequestBuilder {
    merge_same_id_events: Option<bool>,
    include_internal_events: Option<bool>,
}

impl ListTraceEventsQueryRequestBuilder {
    pub fn merge_same_id_events(mut self, value: bool) -> Self {
        self.merge_same_id_events = Some(value);
        self
    }

    pub fn include_internal_events(mut self, value: bool) -> Self {
        self.include_internal_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTraceEventsQueryRequest`].
    pub fn build(self) -> Result<ListTraceEventsQueryRequest, BuildError> {
        Ok(ListTraceEventsQueryRequest {
            merge_same_id_events: self.merge_same_id_events,
            include_internal_events: self.include_internal_events,
        })
    }
}
