pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsageQueryAggregatesOut {
    /// Number of matching customers.
    #[serde(default)]
    pub customer_count: i64,
    /// Total number of distinct API calls.
    #[serde(default)]
    pub total_api_calls: i64,
    /// Total number of matching usage records.
    #[serde(default)]
    pub total_count: i64,
    /// Total usage value across all matching rows.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_value: f64,
    /// Number of matching usage types.
    #[serde(default)]
    pub usage_type_count: i64,
}

impl UsageQueryAggregatesOut {
    pub fn builder() -> UsageQueryAggregatesOutBuilder {
        <UsageQueryAggregatesOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageQueryAggregatesOutBuilder {
    customer_count: Option<i64>,
    total_api_calls: Option<i64>,
    total_count: Option<i64>,
    total_value: Option<f64>,
    usage_type_count: Option<i64>,
}

impl UsageQueryAggregatesOutBuilder {
    pub fn customer_count(mut self, value: i64) -> Self {
        self.customer_count = Some(value);
        self
    }

    pub fn total_api_calls(mut self, value: i64) -> Self {
        self.total_api_calls = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn total_value(mut self, value: f64) -> Self {
        self.total_value = Some(value);
        self
    }

    pub fn usage_type_count(mut self, value: i64) -> Self {
        self.usage_type_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageQueryAggregatesOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_count`](UsageQueryAggregatesOutBuilder::customer_count)
    /// - [`total_api_calls`](UsageQueryAggregatesOutBuilder::total_api_calls)
    /// - [`total_count`](UsageQueryAggregatesOutBuilder::total_count)
    /// - [`total_value`](UsageQueryAggregatesOutBuilder::total_value)
    /// - [`usage_type_count`](UsageQueryAggregatesOutBuilder::usage_type_count)
    pub fn build(self) -> Result<UsageQueryAggregatesOut, BuildError> {
        Ok(UsageQueryAggregatesOut {
            customer_count: self
                .customer_count
                .ok_or_else(|| BuildError::missing_field("customer_count"))?,
            total_api_calls: self
                .total_api_calls
                .ok_or_else(|| BuildError::missing_field("total_api_calls"))?,
            total_count: self
                .total_count
                .ok_or_else(|| BuildError::missing_field("total_count"))?,
            total_value: self
                .total_value
                .ok_or_else(|| BuildError::missing_field("total_value"))?,
            usage_type_count: self
                .usage_type_count
                .ok_or_else(|| BuildError::missing_field("usage_type_count"))?,
        })
    }
}
