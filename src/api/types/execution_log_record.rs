pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionLogRecord {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub span_id: String,
    #[serde(default)]
    pub severity_text: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub log_attributes: HashMap<String, String>,
}

impl ExecutionLogRecord {
    pub fn builder() -> ExecutionLogRecordBuilder {
        <ExecutionLogRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionLogRecordBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    trace_id: Option<String>,
    span_id: Option<String>,
    severity_text: Option<String>,
    body: Option<String>,
    log_attributes: Option<HashMap<String, String>>,
}

impl ExecutionLogRecordBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn severity_text(mut self, value: impl Into<String>) -> Self {
        self.severity_text = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn log_attributes(mut self, value: HashMap<String, String>) -> Self {
        self.log_attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionLogRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](ExecutionLogRecordBuilder::timestamp)
    /// - [`trace_id`](ExecutionLogRecordBuilder::trace_id)
    /// - [`span_id`](ExecutionLogRecordBuilder::span_id)
    /// - [`severity_text`](ExecutionLogRecordBuilder::severity_text)
    /// - [`body`](ExecutionLogRecordBuilder::body)
    /// - [`log_attributes`](ExecutionLogRecordBuilder::log_attributes)
    pub fn build(self) -> Result<ExecutionLogRecord, BuildError> {
        Ok(ExecutionLogRecord {
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            severity_text: self
                .severity_text
                .ok_or_else(|| BuildError::missing_field("severity_text"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            log_attributes: self
                .log_attributes
                .ok_or_else(|| BuildError::missing_field("log_attributes"))?,
        })
    }
}
