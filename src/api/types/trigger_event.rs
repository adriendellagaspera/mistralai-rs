pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TriggerEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub received_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub source_id: String,
    pub status: TriggerStatusValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub trigger_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub triggered_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TriggerEvent {
    pub fn builder() -> TriggerEventBuilder {
        <TriggerEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggerEventBuilder {
    detail: Option<String>,
    event_id: Option<String>,
    received_at: Option<DateTime<FixedOffset>>,
    source_id: Option<String>,
    status: Option<TriggerStatusValue>,
    submitted_at: Option<DateTime<FixedOffset>>,
    trigger_id: Option<String>,
    triggered_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
}

impl TriggerEventBuilder {
    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn received_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.received_at = Some(value);
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: TriggerStatusValue) -> Self {
        self.status = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    pub fn trigger_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_id = Some(value.into());
        self
    }

    pub fn triggered_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.triggered_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TriggerEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](TriggerEventBuilder::event_id)
    /// - [`received_at`](TriggerEventBuilder::received_at)
    /// - [`source_id`](TriggerEventBuilder::source_id)
    /// - [`status`](TriggerEventBuilder::status)
    /// - [`trigger_id`](TriggerEventBuilder::trigger_id)
    pub fn build(self) -> Result<TriggerEvent, BuildError> {
        Ok(TriggerEvent {
            detail: self.detail,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            received_at: self
                .received_at
                .ok_or_else(|| BuildError::missing_field("received_at"))?,
            source_id: self
                .source_id
                .ok_or_else(|| BuildError::missing_field("source_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            submitted_at: self.submitted_at,
            trigger_id: self
                .trigger_id
                .ok_or_else(|| BuildError::missing_field("trigger_id"))?,
            triggered_at: self.triggered_at,
            url: self.url,
        })
    }
}
