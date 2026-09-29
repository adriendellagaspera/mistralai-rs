pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TempoTraceSpan {
    /// The attributes of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TempoTraceAttribute>>,
    /// The end time of the scope in Unix nano
    #[serde(rename = "endTimeUnixNano")]
    #[serde(default)]
    pub end_time_unix_nano: String,
    /// The events of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<TempoTraceEvent>>,
    /// The kind of the scope
    pub kind: TempoTraceScopeKind,
    /// The name of the scope
    #[serde(default)]
    pub name: String,
    /// The parent span ID of the scope
    #[serde(rename = "parentSpanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    /// The span ID of the scope
    #[serde(rename = "spanId")]
    #[serde(default)]
    pub span_id: String,
    /// The start time of the scope in Unix nano
    #[serde(rename = "startTimeUnixNano")]
    #[serde(default)]
    pub start_time_unix_nano: String,
    /// The trace ID of the scope
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
}

impl TempoTraceSpan {
    pub fn builder() -> TempoTraceSpanBuilder {
        <TempoTraceSpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceSpanBuilder {
    attributes: Option<Vec<TempoTraceAttribute>>,
    end_time_unix_nano: Option<String>,
    events: Option<Vec<TempoTraceEvent>>,
    kind: Option<TempoTraceScopeKind>,
    name: Option<String>,
    parent_span_id: Option<String>,
    span_id: Option<String>,
    start_time_unix_nano: Option<String>,
    trace_id: Option<String>,
}

impl TempoTraceSpanBuilder {
    pub fn attributes(mut self, value: Vec<TempoTraceAttribute>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn end_time_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.end_time_unix_nano = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<TempoTraceEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn kind(mut self, value: TempoTraceScopeKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_span_id(mut self, value: impl Into<String>) -> Self {
        self.parent_span_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn start_time_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.start_time_unix_nano = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceSpan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_time_unix_nano`](TempoTraceSpanBuilder::end_time_unix_nano)
    /// - [`kind`](TempoTraceSpanBuilder::kind)
    /// - [`name`](TempoTraceSpanBuilder::name)
    /// - [`span_id`](TempoTraceSpanBuilder::span_id)
    /// - [`start_time_unix_nano`](TempoTraceSpanBuilder::start_time_unix_nano)
    /// - [`trace_id`](TempoTraceSpanBuilder::trace_id)
    pub fn build(self) -> Result<TempoTraceSpan, BuildError> {
        Ok(TempoTraceSpan {
            attributes: self.attributes,
            end_time_unix_nano: self
                .end_time_unix_nano
                .ok_or_else(|| BuildError::missing_field("end_time_unix_nano"))?,
            events: self.events,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_span_id: self.parent_span_id,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            start_time_unix_nano: self
                .start_time_unix_nano
                .ok_or_else(|| BuildError::missing_field("start_time_unix_nano"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
