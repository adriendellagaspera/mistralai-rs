pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResetInvocationBody {
    /// The event ID to reset the workflow execution to
    #[serde(default)]
    pub event_id: i64,
    /// Reason for resetting the workflow execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Whether to exclude signals that happened after the reset point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_signals: Option<bool>,
    /// Whether to exclude updates that happened after the reset point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_updates: Option<bool>,
}

impl ResetInvocationBody {
    pub fn builder() -> ResetInvocationBodyBuilder {
        <ResetInvocationBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResetInvocationBodyBuilder {
    event_id: Option<i64>,
    reason: Option<String>,
    exclude_signals: Option<bool>,
    exclude_updates: Option<bool>,
}

impl ResetInvocationBodyBuilder {
    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn exclude_signals(mut self, value: bool) -> Self {
        self.exclude_signals = Some(value);
        self
    }

    pub fn exclude_updates(mut self, value: bool) -> Self {
        self.exclude_updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResetInvocationBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](ResetInvocationBodyBuilder::event_id)
    pub fn build(self) -> Result<ResetInvocationBody, BuildError> {
        Ok(ResetInvocationBody {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            reason: self.reason,
            exclude_signals: self.exclude_signals,
            exclude_updates: self.exclude_updates,
        })
    }
}
