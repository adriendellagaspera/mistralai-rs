pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioInactivityConfig {
    #[serde(default)]
    pub max_reminders: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    #[serde(default)]
    pub timeout_ms: i64,
}

impl AudioInactivityConfig {
    pub fn builder() -> AudioInactivityConfigBuilder {
        <AudioInactivityConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioInactivityConfigBuilder {
    max_reminders: Option<i64>,
    messages: Option<Vec<String>>,
    timeout_ms: Option<i64>,
}

impl AudioInactivityConfigBuilder {
    pub fn max_reminders(mut self, value: i64) -> Self {
        self.max_reminders = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<String>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn timeout_ms(mut self, value: i64) -> Self {
        self.timeout_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioInactivityConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_reminders`](AudioInactivityConfigBuilder::max_reminders)
    /// - [`timeout_ms`](AudioInactivityConfigBuilder::timeout_ms)
    pub fn build(self) -> Result<AudioInactivityConfig, BuildError> {
        Ok(AudioInactivityConfig {
            max_reminders: self
                .max_reminders
                .ok_or_else(|| BuildError::missing_field("max_reminders"))?,
            messages: self.messages,
            timeout_ms: self
                .timeout_ms
                .ok_or_else(|| BuildError::missing_field("timeout_ms"))?,
        })
    }
}
