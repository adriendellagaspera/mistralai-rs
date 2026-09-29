pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeToolCallsStat {
    #[serde(default)]
    pub approval_always: i64,
    #[serde(default)]
    pub approval_ask: i64,
    #[serde(default)]
    pub approval_never: i64,
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub status_failure: i64,
    #[serde(default)]
    pub status_skipped: i64,
    #[serde(default)]
    pub status_success: i64,
    #[serde(default)]
    pub total: i64,
}

impl VibeToolCallsStat {
    pub fn builder() -> VibeToolCallsStatBuilder {
        <VibeToolCallsStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeToolCallsStatBuilder {
    approval_always: Option<i64>,
    approval_ask: Option<i64>,
    approval_never: Option<i64>,
    day: Option<NaiveDate>,
    status_failure: Option<i64>,
    status_skipped: Option<i64>,
    status_success: Option<i64>,
    total: Option<i64>,
}

impl VibeToolCallsStatBuilder {
    pub fn approval_always(mut self, value: i64) -> Self {
        self.approval_always = Some(value);
        self
    }

    pub fn approval_ask(mut self, value: i64) -> Self {
        self.approval_ask = Some(value);
        self
    }

    pub fn approval_never(mut self, value: i64) -> Self {
        self.approval_never = Some(value);
        self
    }

    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn status_failure(mut self, value: i64) -> Self {
        self.status_failure = Some(value);
        self
    }

    pub fn status_skipped(mut self, value: i64) -> Self {
        self.status_skipped = Some(value);
        self
    }

    pub fn status_success(mut self, value: i64) -> Self {
        self.status_success = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeToolCallsStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval_always`](VibeToolCallsStatBuilder::approval_always)
    /// - [`approval_ask`](VibeToolCallsStatBuilder::approval_ask)
    /// - [`approval_never`](VibeToolCallsStatBuilder::approval_never)
    /// - [`day`](VibeToolCallsStatBuilder::day)
    /// - [`status_failure`](VibeToolCallsStatBuilder::status_failure)
    /// - [`status_skipped`](VibeToolCallsStatBuilder::status_skipped)
    /// - [`status_success`](VibeToolCallsStatBuilder::status_success)
    /// - [`total`](VibeToolCallsStatBuilder::total)
    pub fn build(self) -> Result<VibeToolCallsStat, BuildError> {
        Ok(VibeToolCallsStat {
            approval_always: self
                .approval_always
                .ok_or_else(|| BuildError::missing_field("approval_always"))?,
            approval_ask: self
                .approval_ask
                .ok_or_else(|| BuildError::missing_field("approval_ask"))?,
            approval_never: self
                .approval_never
                .ok_or_else(|| BuildError::missing_field("approval_never"))?,
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            status_failure: self
                .status_failure
                .ok_or_else(|| BuildError::missing_field("status_failure"))?,
            status_skipped: self
                .status_skipped
                .ok_or_else(|| BuildError::missing_field("status_skipped"))?,
            status_success: self
                .status_success
                .ok_or_else(|| BuildError::missing_field("status_success"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
