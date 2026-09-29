pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeploymentLogRecord {
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

impl DeploymentLogRecord {
    pub fn builder() -> DeploymentLogRecordBuilder {
        <DeploymentLogRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentLogRecordBuilder {
    body: Option<String>,
    log_attributes: Option<HashMap<String, String>>,
    severity_text: Option<String>,
    span_id: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    trace_id: Option<String>,
}

impl DeploymentLogRecordBuilder {
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

    /// Consumes the builder and constructs a [`DeploymentLogRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](DeploymentLogRecordBuilder::body)
    /// - [`log_attributes`](DeploymentLogRecordBuilder::log_attributes)
    /// - [`severity_text`](DeploymentLogRecordBuilder::severity_text)
    /// - [`span_id`](DeploymentLogRecordBuilder::span_id)
    /// - [`timestamp`](DeploymentLogRecordBuilder::timestamp)
    /// - [`trace_id`](DeploymentLogRecordBuilder::trace_id)
    pub fn build(self) -> Result<DeploymentLogRecord, BuildError> {
        Ok(DeploymentLogRecord {
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
