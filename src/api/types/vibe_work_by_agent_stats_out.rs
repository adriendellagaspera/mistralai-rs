pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeWorkByAgentStatsOut {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub data: Vec<VibeWorkByAgentStat>,
}

impl VibeWorkByAgentStatsOut {
    pub fn builder() -> VibeWorkByAgentStatsOutBuilder {
        <VibeWorkByAgentStatsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeWorkByAgentStatsOutBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    data: Option<Vec<VibeWorkByAgentStat>>,
}

impl VibeWorkByAgentStatsOutBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<VibeWorkByAgentStat>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeWorkByAgentStatsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](VibeWorkByAgentStatsOutBuilder::start_time)
    /// - [`end_time`](VibeWorkByAgentStatsOutBuilder::end_time)
    /// - [`data`](VibeWorkByAgentStatsOutBuilder::data)
    pub fn build(self) -> Result<VibeWorkByAgentStatsOut, BuildError> {
        Ok(VibeWorkByAgentStatsOut {
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
