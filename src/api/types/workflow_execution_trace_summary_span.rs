pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionTraceSummarySpan {
    /// The attributes of the span
    #[serde(default)]
    pub attributes: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    /// The child spans of the span
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Box<WorkflowExecutionTraceSummarySpan>>>,
    /// The end time of the span in nanoseconds since the Unix epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_unix_nano: Option<i64>,
    /// The events of the span
    #[serde(default)]
    pub events: Vec<WorkflowExecutionTraceEvent>,
    /// The name of the span
    #[serde(default)]
    pub name: String,
    /// The ID of the span
    #[serde(default)]
    pub span_id: String,
    /// The start time of the span in nanoseconds since the Unix epoch
    #[serde(default)]
    pub start_time_unix_nano: i64,
}

impl WorkflowExecutionTraceSummarySpan {
    pub fn builder() -> WorkflowExecutionTraceSummarySpanBuilder {
        <WorkflowExecutionTraceSummarySpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionTraceSummarySpanBuilder {
    attributes: Option<HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>>,
    children: Option<Vec<Box<WorkflowExecutionTraceSummarySpan>>>,
    end_time_unix_nano: Option<i64>,
    events: Option<Vec<WorkflowExecutionTraceEvent>>,
    name: Option<String>,
    span_id: Option<String>,
    start_time_unix_nano: Option<i64>,
}

impl WorkflowExecutionTraceSummarySpanBuilder {
    pub fn attributes(
        mut self,
        value: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn children(mut self, value: Vec<Box<WorkflowExecutionTraceSummarySpan>>) -> Self {
        self.children = Some(value);
        self
    }

    pub fn end_time_unix_nano(mut self, value: i64) -> Self {
        self.end_time_unix_nano = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<WorkflowExecutionTraceEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn start_time_unix_nano(mut self, value: i64) -> Self {
        self.start_time_unix_nano = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionTraceSummarySpan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](WorkflowExecutionTraceSummarySpanBuilder::attributes)
    /// - [`events`](WorkflowExecutionTraceSummarySpanBuilder::events)
    /// - [`name`](WorkflowExecutionTraceSummarySpanBuilder::name)
    /// - [`span_id`](WorkflowExecutionTraceSummarySpanBuilder::span_id)
    /// - [`start_time_unix_nano`](WorkflowExecutionTraceSummarySpanBuilder::start_time_unix_nano)
    pub fn build(self) -> Result<WorkflowExecutionTraceSummarySpan, BuildError> {
        Ok(WorkflowExecutionTraceSummarySpan {
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            children: self.children,
            end_time_unix_nano: self.end_time_unix_nano,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            start_time_unix_nano: self
                .start_time_unix_nano
                .ok_or_else(|| BuildError::missing_field("start_time_unix_nano"))?,
        })
    }
}
