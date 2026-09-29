pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionProgressTraceEvent {
    /// The attributes of the event
    #[serde(default)]
    pub attributes: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    /// The end time of the event in milliseconds since the Unix epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_unix_ms: Option<i64>,
    /// The error message, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The ID of the event
    #[serde(default)]
    pub id: String,
    /// Whether the event is internal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Name of the event
    #[serde(default)]
    pub name: String,
    /// The start time of the event in milliseconds since the Unix epoch
    #[serde(default)]
    pub start_time_unix_ms: i64,
    /// The progress message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventProgressStatus>,
    /// The timestamp of the event in nanoseconds since the Unix epoch
    #[serde(default)]
    pub timestamp_unix_nano: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EventType>,
}

impl WorkflowExecutionProgressTraceEvent {
    pub fn builder() -> WorkflowExecutionProgressTraceEventBuilder {
        <WorkflowExecutionProgressTraceEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionProgressTraceEventBuilder {
    attributes: Option<HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>>,
    end_time_unix_ms: Option<i64>,
    error: Option<String>,
    id: Option<String>,
    internal: Option<bool>,
    name: Option<String>,
    start_time_unix_ms: Option<i64>,
    status: Option<EventProgressStatus>,
    timestamp_unix_nano: Option<i64>,
    r#type: Option<EventType>,
}

impl WorkflowExecutionProgressTraceEventBuilder {
    pub fn attributes(
        mut self,
        value: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn end_time_unix_ms(mut self, value: i64) -> Self {
        self.end_time_unix_ms = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn internal(mut self, value: bool) -> Self {
        self.internal = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn start_time_unix_ms(mut self, value: i64) -> Self {
        self.start_time_unix_ms = Some(value);
        self
    }

    pub fn status(mut self, value: EventProgressStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn timestamp_unix_nano(mut self, value: i64) -> Self {
        self.timestamp_unix_nano = Some(value);
        self
    }

    pub fn r#type(mut self, value: EventType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionProgressTraceEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](WorkflowExecutionProgressTraceEventBuilder::attributes)
    /// - [`id`](WorkflowExecutionProgressTraceEventBuilder::id)
    /// - [`name`](WorkflowExecutionProgressTraceEventBuilder::name)
    /// - [`start_time_unix_ms`](WorkflowExecutionProgressTraceEventBuilder::start_time_unix_ms)
    /// - [`timestamp_unix_nano`](WorkflowExecutionProgressTraceEventBuilder::timestamp_unix_nano)
    pub fn build(self) -> Result<WorkflowExecutionProgressTraceEvent, BuildError> {
        Ok(WorkflowExecutionProgressTraceEvent {
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            end_time_unix_ms: self.end_time_unix_ms,
            error: self.error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            internal: self.internal,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            start_time_unix_ms: self
                .start_time_unix_ms
                .ok_or_else(|| BuildError::missing_field("start_time_unix_ms"))?,
            status: self.status,
            timestamp_unix_nano: self
                .timestamp_unix_nano
                .ok_or_else(|| BuildError::missing_field("timestamp_unix_nano"))?,
            r#type: self.r#type,
        })
    }
}
