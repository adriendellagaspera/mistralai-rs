pub use crate::prelude::*;

/// Query parameters for agent_stats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentStatsQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
}

impl AgentStatsQueryRequest {
    pub fn builder() -> AgentStatsQueryRequestBuilder {
        <AgentStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentStatsQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
}

impl AgentStatsQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](AgentStatsQueryRequestBuilder::start_time)
    /// - [`end_time`](AgentStatsQueryRequestBuilder::end_time)
    pub fn build(self) -> Result<AgentStatsQueryRequest, BuildError> {
        Ok(AgentStatsQueryRequest {
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
        })
    }
}
