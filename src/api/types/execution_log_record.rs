pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionLogRecord {
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub log_attributes: HashMap<String, String>,
    #[serde(default)]
    pub severity_text: String,
    #[serde(default)]
    pub span_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    #[serde(default)]
    pub trace_id: String,
}

impl ExecutionLogRecord {
    pub fn builder() -> ExecutionLogRecordBuilder {
        <ExecutionLogRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionLogRecordBuilder {
    body: Option<String>,
    log_attributes: Option<HashMap<String, String>>,
    severity_text: Option<String>,
    span_id: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    trace_id: Option<String>,
}

impl ExecutionLogRecordBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn log_attributes(mut self, value: HashMap<String, String>) -> Self {
        self.log_attributes = Some(value);
        self
    }

    pub fn severity_text(mut self, value: impl Into<String>) -> Self {
        self.severity_text = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionLogRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](ExecutionLogRecordBuilder::body)
    /// - [`log_attributes`](ExecutionLogRecordBuilder::log_attributes)
    /// - [`severity_text`](ExecutionLogRecordBuilder::severity_text)
    /// - [`span_id`](ExecutionLogRecordBuilder::span_id)
    /// - [`timestamp`](ExecutionLogRecordBuilder::timestamp)
    /// - [`trace_id`](ExecutionLogRecordBuilder::trace_id)
    pub fn build(self) -> Result<ExecutionLogRecord, BuildError> {
        Ok(ExecutionLogRecord {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            log_attributes: self
                .log_attributes
                .ok_or_else(|| BuildError::missing_field("log_attributes"))?,
            severity_text: self
                .severity_text
                .ok_or_else(|| BuildError::missing_field("severity_text"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
