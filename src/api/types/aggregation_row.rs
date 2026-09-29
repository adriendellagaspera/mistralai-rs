pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AggregationRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub metric_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_value: Option<AggregationRowMetricValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_bucket: Option<DateTime<FixedOffset>>,
}

impl AggregationRow {
    pub fn builder() -> AggregationRowBuilder {
        <AggregationRowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregationRowBuilder {
    dimensions: Option<HashMap<String, serde_json::Value>>,
    metric_name: Option<String>,
    metric_value: Option<AggregationRowMetricValue>,
    time_bucket: Option<DateTime<FixedOffset>>,
}

impl AggregationRowBuilder {
    pub fn dimensions(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn metric_name(mut self, value: impl Into<String>) -> Self {
        self.metric_name = Some(value.into());
        self
    }

    pub fn metric_value(mut self, value: AggregationRowMetricValue) -> Self {
        self.metric_value = Some(value);
        self
    }

    pub fn time_bucket(mut self, value: DateTime<FixedOffset>) -> Self {
        self.time_bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AggregationRow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metric_name`](AggregationRowBuilder::metric_name)
    pub fn build(self) -> Result<AggregationRow, BuildError> {
        Ok(AggregationRow {
            dimensions: self.dimensions,
            metric_name: self
                .metric_name
                .ok_or_else(|| BuildError::missing_field("metric_name"))?,
            metric_value: self.metric_value,
            time_bucket: self.time_bucket,
        })
    }
}
