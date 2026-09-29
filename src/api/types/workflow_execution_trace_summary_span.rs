pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionTraceSummarySpan {
    /// The ID of the span
    #[serde(default)]
    pub span_id: String,
    /// The name of the span
    #[serde(default)]
    pub name: String,
    /// The start time of the span in nanoseconds since the Unix epoch
    #[serde(default)]
    pub start_time_unix_nano: i64,
    /// The end time of the span in nanoseconds since the Unix epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_unix_nano: Option<i64>,
    /// The attributes of the span
    #[serde(default)]
    pub attributes: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    /// The events of the span
    #[serde(default)]
    pub events: Vec<WorkflowExecutionTraceEvent>,
    /// The child spans of the span
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Box<WorkflowExecutionTraceSummarySpan>>>,
}

impl WorkflowExecutionTraceSummarySpan {
    pub fn builder() -> WorkflowExecutionTraceSummarySpanBuilder {
        <WorkflowExecutionTraceSummarySpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionTraceSummarySpanBuilder {
    span_id: Option<String>,
    name: Option<String>,
    start_time_unix_nano: Option<i64>,
    end_time_unix_nano: Option<i64>,
    attributes: Option<HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>>,
    events: Option<Vec<WorkflowExecutionTraceEvent>>,
    children: Option<Vec<Box<WorkflowExecutionTraceSummarySpan>>>,
}

impl WorkflowExecutionTraceSummarySpanBuilder {
    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn start_time_unix_nano(mut self, value: i64) -> Self {
        self.start_time_unix_nano = Some(value);
        self
    }

    pub fn end_time_unix_nano(mut self, value: i64) -> Self {
        self.end_time_unix_nano = Some(value);
        self
    }

    pub fn attributes(
        mut self,
        value: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<WorkflowExecutionTraceEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn children(mut self, value: Vec<Box<WorkflowExecutionTraceSummarySpan>>) -> Self {
        self.children = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionTraceSummarySpan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`span_id`](WorkflowExecutionTraceSummarySpanBuilder::span_id)
    /// - [`name`](WorkflowExecutionTraceSummarySpanBuilder::name)
    /// - [`start_time_unix_nano`](WorkflowExecutionTraceSummarySpanBuilder::start_time_unix_nano)
    /// - [`attributes`](WorkflowExecutionTraceSummarySpanBuilder::attributes)
    /// - [`events`](WorkflowExecutionTraceSummarySpanBuilder::events)
    pub fn build(self) -> Result<WorkflowExecutionTraceSummarySpan, BuildError> {
        Ok(WorkflowExecutionTraceSummarySpan {
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            start_time_unix_nano: self
                .start_time_unix_nano
                .ok_or_else(|| BuildError::missing_field("start_time_unix_nano"))?,
            end_time_unix_nano: self.end_time_unix_nano,
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            children: self.children,
        })
    }
}
