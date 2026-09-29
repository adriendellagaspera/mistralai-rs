pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionTraceEvent {
    /// The attributes of the event
    #[serde(default)]
    pub attributes: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    /// The ID of the event
    #[serde(default)]
    pub id: String,
    /// Whether the event is internal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Name of the event
    #[serde(default)]
    pub name: String,
    /// The timestamp of the event in nanoseconds since the Unix epoch
    #[serde(default)]
    pub timestamp_unix_nano: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EventType>,
}

impl WorkflowExecutionTraceEvent {
    pub fn builder() -> WorkflowExecutionTraceEventBuilder {
        <WorkflowExecutionTraceEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionTraceEventBuilder {
    attributes: Option<HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>>,
    id: Option<String>,
    internal: Option<bool>,
    name: Option<String>,
    timestamp_unix_nano: Option<i64>,
    r#type: Option<EventType>,
}

impl WorkflowExecutionTraceEventBuilder {
    pub fn attributes(
        mut self,
        value: HashMap<String, Option<WorkflowExecutionTraceSummaryAttributesValues>>,
    ) -> Self {
        self.attributes = Some(value);
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

    pub fn timestamp_unix_nano(mut self, value: i64) -> Self {
        self.timestamp_unix_nano = Some(value);
        self
    }

    pub fn r#type(mut self, value: EventType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionTraceEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](WorkflowExecutionTraceEventBuilder::attributes)
    /// - [`id`](WorkflowExecutionTraceEventBuilder::id)
    /// - [`name`](WorkflowExecutionTraceEventBuilder::name)
    /// - [`timestamp_unix_nano`](WorkflowExecutionTraceEventBuilder::timestamp_unix_nano)
    pub fn build(self) -> Result<WorkflowExecutionTraceEvent, BuildError> {
        Ok(WorkflowExecutionTraceEvent {
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            internal: self.internal,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            timestamp_unix_nano: self
                .timestamp_unix_nano
                .ok_or_else(|| BuildError::missing_field("timestamp_unix_nano"))?,
            r#type: self.r#type,
        })
    }
}
