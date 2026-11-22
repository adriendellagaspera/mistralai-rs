pub use crate::prelude::*;

/// Query parameters for organization_stats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationStatsQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
}

impl OrganizationStatsQueryRequest {
    pub fn builder() -> OrganizationStatsQueryRequestBuilder {
        <OrganizationStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationStatsQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
}

impl OrganizationStatsQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](OrganizationStatsQueryRequestBuilder::start_time)
    /// - [`end_time`](OrganizationStatsQueryRequestBuilder::end_time)
    pub fn build(self) -> Result<OrganizationStatsQueryRequest, BuildError> {
        Ok(OrganizationStatsQueryRequest {
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
        })
    }
}
