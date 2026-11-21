pub use crate::prelude::*;

/// Query parameters for user_stats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserStatsQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
}

impl UserStatsQueryRequest {
    pub fn builder() -> UserStatsQueryRequestBuilder {
        <UserStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserStatsQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
}

impl UserStatsQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](UserStatsQueryRequestBuilder::start_time)
    /// - [`end_time`](UserStatsQueryRequestBuilder::end_time)
    pub fn build(self) -> Result<UserStatsQueryRequest, BuildError> {
        Ok(UserStatsQueryRequest {
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
        })
    }
}
