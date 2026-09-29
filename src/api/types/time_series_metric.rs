pub use crate::prelude::*;

/// Time-series metric with timestamp-value pairs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimeSeriesMetric {
    #[serde(default)]
    pub value: Vec<Vec<serde_json::Value>>,
}

impl TimeSeriesMetric {
    pub fn builder() -> TimeSeriesMetricBuilder {
        <TimeSeriesMetricBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeSeriesMetricBuilder {
    value: Option<Vec<Vec<serde_json::Value>>>,
}

impl TimeSeriesMetricBuilder {
    pub fn value(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimeSeriesMetric`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](TimeSeriesMetricBuilder::value)
    pub fn build(self) -> Result<TimeSeriesMetric, BuildError> {
        Ok(TimeSeriesMetric {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
