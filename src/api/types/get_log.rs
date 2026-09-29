pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLog {
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub customer_id: String,
    #[serde(default)]
    pub event_name: String,
    #[serde(default)]
    pub log_attributes: HashMap<String, String>,
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub resource_attributes: HashMap<String, String>,
    #[serde(default)]
    pub resource_schema_url: String,
    #[serde(default)]
    pub scope_attributes: HashMap<String, String>,
    #[serde(default)]
    pub scope_name: String,
    #[serde(default)]
    pub scope_schema_url: String,
    #[serde(default)]
    pub scope_version: String,
    #[serde(default)]
    pub service_name: String,
    #[serde(default)]
    pub severity_number: i64,
    #[serde(default)]
    pub severity_text: String,
    #[serde(default)]
    pub span_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    #[serde(default)]
    pub trace_flags: i64,
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub workspace_id: String,
}

impl GetLog {
    pub fn builder() -> GetLogBuilder {
        <GetLogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLogBuilder {
    body: Option<String>,
    customer_id: Option<String>,
    event_name: Option<String>,
    log_attributes: Option<HashMap<String, String>>,
    organization_id: Option<String>,
    resource_attributes: Option<HashMap<String, String>>,
    resource_schema_url: Option<String>,
    scope_attributes: Option<HashMap<String, String>>,
    scope_name: Option<String>,
    scope_schema_url: Option<String>,
    scope_version: Option<String>,
    service_name: Option<String>,
    severity_number: Option<i64>,
    severity_text: Option<String>,
    span_id: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    trace_flags: Option<i64>,
    trace_id: Option<String>,
    user_id: Option<String>,
    workspace_id: Option<String>,
}

impl GetLogBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn log_attributes(mut self, value: HashMap<String, String>) -> Self {
        self.log_attributes = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn resource_attributes(mut self, value: HashMap<String, String>) -> Self {
        self.resource_attributes = Some(value);
        self
    }

    pub fn resource_schema_url(mut self, value: impl Into<String>) -> Self {
        self.resource_schema_url = Some(value.into());
        self
    }

    pub fn scope_attributes(mut self, value: HashMap<String, String>) -> Self {
        self.scope_attributes = Some(value);
        self
    }

    pub fn scope_name(mut self, value: impl Into<String>) -> Self {
        self.scope_name = Some(value.into());
        self
    }

    pub fn scope_schema_url(mut self, value: impl Into<String>) -> Self {
        self.scope_schema_url = Some(value.into());
        self
    }

    pub fn scope_version(mut self, value: impl Into<String>) -> Self {
        self.scope_version = Some(value.into());
        self
    }

    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    pub fn severity_number(mut self, value: i64) -> Self {
        self.severity_number = Some(value);
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

    pub fn trace_flags(mut self, value: i64) -> Self {
        self.trace_flags = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLog`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](GetLogBuilder::body)
    /// - [`customer_id`](GetLogBuilder::customer_id)
    /// - [`event_name`](GetLogBuilder::event_name)
    /// - [`log_attributes`](GetLogBuilder::log_attributes)
    /// - [`organization_id`](GetLogBuilder::organization_id)
    /// - [`resource_attributes`](GetLogBuilder::resource_attributes)
    /// - [`resource_schema_url`](GetLogBuilder::resource_schema_url)
    /// - [`scope_attributes`](GetLogBuilder::scope_attributes)
    /// - [`scope_name`](GetLogBuilder::scope_name)
    /// - [`scope_schema_url`](GetLogBuilder::scope_schema_url)
    /// - [`scope_version`](GetLogBuilder::scope_version)
    /// - [`service_name`](GetLogBuilder::service_name)
    /// - [`severity_number`](GetLogBuilder::severity_number)
    /// - [`severity_text`](GetLogBuilder::severity_text)
    /// - [`span_id`](GetLogBuilder::span_id)
    /// - [`timestamp`](GetLogBuilder::timestamp)
    /// - [`trace_flags`](GetLogBuilder::trace_flags)
    /// - [`trace_id`](GetLogBuilder::trace_id)
    /// - [`user_id`](GetLogBuilder::user_id)
    /// - [`workspace_id`](GetLogBuilder::workspace_id)
    pub fn build(self) -> Result<GetLog, BuildError> {
        Ok(GetLog {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            log_attributes: self
                .log_attributes
                .ok_or_else(|| BuildError::missing_field("log_attributes"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            resource_attributes: self
                .resource_attributes
                .ok_or_else(|| BuildError::missing_field("resource_attributes"))?,
            resource_schema_url: self
                .resource_schema_url
                .ok_or_else(|| BuildError::missing_field("resource_schema_url"))?,
            scope_attributes: self
                .scope_attributes
                .ok_or_else(|| BuildError::missing_field("scope_attributes"))?,
            scope_name: self
                .scope_name
                .ok_or_else(|| BuildError::missing_field("scope_name"))?,
            scope_schema_url: self
                .scope_schema_url
                .ok_or_else(|| BuildError::missing_field("scope_schema_url"))?,
            scope_version: self
                .scope_version
                .ok_or_else(|| BuildError::missing_field("scope_version"))?,
            service_name: self
                .service_name
                .ok_or_else(|| BuildError::missing_field("service_name"))?,
            severity_number: self
                .severity_number
                .ok_or_else(|| BuildError::missing_field("severity_number"))?,
            severity_text: self
                .severity_text
                .ok_or_else(|| BuildError::missing_field("severity_text"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            trace_flags: self
                .trace_flags
                .ok_or_else(|| BuildError::missing_field("trace_flags"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
