pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceEvent {
    /// The name of the event
    #[serde(default)]
    pub name: String,
    /// The time of the event in Unix nano
    #[serde(rename = "timeUnixNano")]
    #[serde(default)]
    pub time_unix_nano: String,
    /// The attributes of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TempoTraceAttribute>>,
}

impl TempoTraceEvent {
    pub fn builder() -> TempoTraceEventBuilder {
        <TempoTraceEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceEventBuilder {
    name: Option<String>,
    time_unix_nano: Option<String>,
    attributes: Option<Vec<TempoTraceAttribute>>,
}

impl TempoTraceEventBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn time_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.time_unix_nano = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: Vec<TempoTraceAttribute>) -> Self {
        self.attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TempoTraceEventBuilder::name)
    /// - [`time_unix_nano`](TempoTraceEventBuilder::time_unix_nano)
    pub fn build(self) -> Result<TempoTraceEvent, BuildError> {
        Ok(TempoTraceEvent {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            time_unix_nano: self
                .time_unix_nano
                .ok_or_else(|| BuildError::missing_field("time_unix_nano"))?,
            attributes: self.attributes,
        })
    }
}
