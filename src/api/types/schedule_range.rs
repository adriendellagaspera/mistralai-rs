pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleRange {
    #[serde(default)]
    pub start: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
}

impl ScheduleRange {
    pub fn builder() -> ScheduleRangeBuilder {
        <ScheduleRangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleRangeBuilder {
    start: Option<i64>,
    end: Option<i64>,
    step: Option<i64>,
}

impl ScheduleRangeBuilder {
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn step(mut self, value: i64) -> Self {
        self.step = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleRange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](ScheduleRangeBuilder::start)
    pub fn build(self) -> Result<ScheduleRange, BuildError> {
        Ok(ScheduleRange {
            start: self
                .start
                .ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end,
            step: self.step,
        })
    }
}
