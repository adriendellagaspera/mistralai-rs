pub use crate::prelude::*;

/// Query parameters for get_stream_events_v1_workflows_events_stream_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetStreamEventsV1WorkflowsEventsStreamGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<GetStreamEventsV1WorkflowsEventsStreamGetEventsRequestScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_exec_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_workflow_exec_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_workflow_exec_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seq: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_filters: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_event_types: Option<Vec<WorkflowEventType>>,
}

impl GetStreamEventsV1WorkflowsEventsStreamGetQueryRequest {
    pub fn builder() -> GetStreamEventsV1WorkflowsEventsStreamGetQueryRequestBuilder {
        <GetStreamEventsV1WorkflowsEventsStreamGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStreamEventsV1WorkflowsEventsStreamGetQueryRequestBuilder {
    scope: Option<GetStreamEventsV1WorkflowsEventsStreamGetEventsRequestScope>,
    activity_name: Option<String>,
    activity_id: Option<String>,
    workflow_name: Option<String>,
    workflow_exec_id: Option<String>,
    root_workflow_exec_id: Option<String>,
    parent_workflow_exec_id: Option<String>,
    stream: Option<String>,
    start_seq: Option<i64>,
    metadata_filters: Option<HashMap<String, serde_json::Value>>,
    workflow_event_types: Option<Vec<WorkflowEventType>>,
}

impl GetStreamEventsV1WorkflowsEventsStreamGetQueryRequestBuilder {
    pub fn scope(
        mut self,
        value: GetStreamEventsV1WorkflowsEventsStreamGetEventsRequestScope,
    ) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn activity_name(mut self, value: impl Into<String>) -> Self {
        self.activity_name = Some(value.into());
        self
    }

    pub fn activity_id(mut self, value: impl Into<String>) -> Self {
        self.activity_id = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_exec_id = Some(value.into());
        self
    }

    pub fn root_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.root_workflow_exec_id = Some(value.into());
        self
    }

    pub fn parent_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.parent_workflow_exec_id = Some(value.into());
        self
    }

    pub fn stream(mut self, value: impl Into<String>) -> Self {
        self.stream = Some(value.into());
        self
    }

    pub fn start_seq(mut self, value: i64) -> Self {
        self.start_seq = Some(value);
        self
    }

    pub fn metadata_filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata_filters = Some(value);
        self
    }

    pub fn workflow_event_types(mut self, value: Vec<WorkflowEventType>) -> Self {
        self.workflow_event_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetStreamEventsV1WorkflowsEventsStreamGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetStreamEventsV1WorkflowsEventsStreamGetQueryRequest, BuildError> {
        Ok(GetStreamEventsV1WorkflowsEventsStreamGetQueryRequest {
            scope: self.scope,
            activity_name: self.activity_name,
            activity_id: self.activity_id,
            workflow_name: self.workflow_name,
            workflow_exec_id: self.workflow_exec_id,
            root_workflow_exec_id: self.root_workflow_exec_id,
            parent_workflow_exec_id: self.parent_workflow_exec_id,
            stream: self.stream,
            start_seq: self.start_seq,
            metadata_filters: self.metadata_filters,
            workflow_event_types: self.workflow_event_types,
        })
    }
}
