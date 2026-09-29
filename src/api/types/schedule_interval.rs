pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleInterval {
    #[serde(default)]
    pub every: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}

impl ScheduleInterval {
    pub fn builder() -> ScheduleIntervalBuilder {
        <ScheduleIntervalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleIntervalBuilder {
    every: Option<String>,
    offset: Option<String>,
}

impl ScheduleIntervalBuilder {
    pub fn every(mut self, value: impl Into<String>) -> Self {
        self.every = Some(value.into());
        self
    }

    pub fn offset(mut self, value: impl Into<String>) -> Self {
        self.offset = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleInterval`].
    /// This method will fail if any of the following fields are not set:
    /// - [`every`](ScheduleIntervalBuilder::every)
    pub fn build(self) -> Result<ScheduleInterval, BuildError> {
        Ok(ScheduleInterval {
            every: self
                .every
                .ok_or_else(|| BuildError::missing_field("every"))?,
            offset: self.offset,
        })
    }
}
