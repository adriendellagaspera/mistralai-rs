pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeWorkByTimeStatsOut {
    #[serde(default)]
    pub data: Vec<VibeWorkByTimeStat>,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
}

impl VibeWorkByTimeStatsOut {
    pub fn builder() -> VibeWorkByTimeStatsOutBuilder {
        <VibeWorkByTimeStatsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeWorkByTimeStatsOutBuilder {
    data: Option<Vec<VibeWorkByTimeStat>>,
    end_time: Option<i64>,
    start_time: Option<i64>,
}

impl VibeWorkByTimeStatsOutBuilder {
    pub fn data(mut self, value: Vec<VibeWorkByTimeStat>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeWorkByTimeStatsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](VibeWorkByTimeStatsOutBuilder::data)
    /// - [`end_time`](VibeWorkByTimeStatsOutBuilder::end_time)
    /// - [`start_time`](VibeWorkByTimeStatsOutBuilder::start_time)
    pub fn build(self) -> Result<VibeWorkByTimeStatsOut, BuildError> {
        Ok(VibeWorkByTimeStatsOut {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
        })
    }
}
