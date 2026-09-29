pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionTraceEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EventType>,
    /// Name of the event
    #[serde(default)]
    pub name: String,
    /// The ID of the event
    #[serde(default)]
    pub id: String,
    /// The timestamp of the event in nanoseconds since the Unix epoch
    #[serde(default)]
    pub timestamp_unix_nano: i64,
    /// The attributes of the event
    #[serde(default)]
    pub attributes: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    /// Whether the event is internal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
}

impl WorkflowExecutionTraceEvent {
    pub fn builder() -> WorkflowExecutionTraceEventBuilder {
        <WorkflowExecutionTraceEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionTraceEventBuilder {
    r#type: Option<EventType>,
    name: Option<String>,
    id: Option<String>,
    timestamp_unix_nano: Option<i64>,
    attributes: Option<HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>>,
    internal: Option<bool>,
}

impl WorkflowExecutionTraceEventBuilder {
    pub fn r#type(mut self, value: EventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn timestamp_unix_nano(mut self, value: i64) -> Self {
        self.timestamp_unix_nano = Some(value);
        self
    }

    pub fn attributes(
        mut self,
        value: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn internal(mut self, value: bool) -> Self {
        self.internal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionTraceEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WorkflowExecutionTraceEventBuilder::name)
    /// - [`id`](WorkflowExecutionTraceEventBuilder::id)
    /// - [`timestamp_unix_nano`](WorkflowExecutionTraceEventBuilder::timestamp_unix_nano)
    /// - [`attributes`](WorkflowExecutionTraceEventBuilder::attributes)
    pub fn build(self) -> Result<WorkflowExecutionTraceEvent, BuildError> {
        Ok(WorkflowExecutionTraceEvent {
            r#type: self.r#type,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            timestamp_unix_nano: self
                .timestamp_unix_nano
                .ok_or_else(|| BuildError::missing_field("timestamp_unix_nano"))?,
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            internal: self.internal,
        })
    }
}
