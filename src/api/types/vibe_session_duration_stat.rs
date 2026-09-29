pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VibeSessionDurationStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_duration_hours: f64,
}

impl VibeSessionDurationStat {
    pub fn builder() -> VibeSessionDurationStatBuilder {
        <VibeSessionDurationStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeSessionDurationStatBuilder {
    day: Option<NaiveDate>,
    total_duration_hours: Option<f64>,
}

impl VibeSessionDurationStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn total_duration_hours(mut self, value: f64) -> Self {
        self.total_duration_hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeSessionDurationStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeSessionDurationStatBuilder::day)
    /// - [`total_duration_hours`](VibeSessionDurationStatBuilder::total_duration_hours)
    pub fn build(self) -> Result<VibeSessionDurationStat, BuildError> {
        Ok(VibeSessionDurationStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            total_duration_hours: self
                .total_duration_hours
                .ok_or_else(|| BuildError::missing_field("total_duration_hours"))?,
        })
    }
}
