pub use crate::prelude::*;

/// Query parameters for time_stats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeStatsQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    /// Time interval used to group usage results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<TimeStatsVibeWorkAnalyticsRequestGranularity>,
}

impl TimeStatsQueryRequest {
    pub fn builder() -> TimeStatsQueryRequestBuilder {
        <TimeStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeStatsQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    granularity: Option<TimeStatsVibeWorkAnalyticsRequestGranularity>,
}

impl TimeStatsQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn granularity(mut self, value: TimeStatsVibeWorkAnalyticsRequestGranularity) -> Self {
        self.granularity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimeStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](TimeStatsQueryRequestBuilder::start_time)
    /// - [`end_time`](TimeStatsQueryRequestBuilder::end_time)
    pub fn build(self) -> Result<TimeStatsQueryRequest, BuildError> {
        Ok(TimeStatsQueryRequest {
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
            granularity: self.granularity,
        })
    }
}
