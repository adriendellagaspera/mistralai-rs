pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeWorkByUserStatsOut {
    #[serde(default)]
    pub data: Vec<VibeWorkByUserStat>,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
}

impl VibeWorkByUserStatsOut {
    pub fn builder() -> VibeWorkByUserStatsOutBuilder {
        <VibeWorkByUserStatsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeWorkByUserStatsOutBuilder {
    data: Option<Vec<VibeWorkByUserStat>>,
    end_time: Option<i64>,
    start_time: Option<i64>,
}

impl VibeWorkByUserStatsOutBuilder {
    pub fn data(mut self, value: Vec<VibeWorkByUserStat>) -> Self {
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

    /// Consumes the builder and constructs a [`VibeWorkByUserStatsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](VibeWorkByUserStatsOutBuilder::data)
    /// - [`end_time`](VibeWorkByUserStatsOutBuilder::end_time)
    /// - [`start_time`](VibeWorkByUserStatsOutBuilder::start_time)
    pub fn build(self) -> Result<VibeWorkByUserStatsOut, BuildError> {
        Ok(VibeWorkByUserStatsOut {
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
