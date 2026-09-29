pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AggregationMeta {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub from_timestamp: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity_seconds: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub to_timestamp: DateTime<FixedOffset>,
}

impl AggregationMeta {
    pub fn builder() -> AggregationMetaBuilder {
        <AggregationMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregationMetaBuilder {
    from_timestamp: Option<DateTime<FixedOffset>>,
    granularity_seconds: Option<i64>,
    to_timestamp: Option<DateTime<FixedOffset>>,
}

impl AggregationMetaBuilder {
    pub fn from_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from_timestamp = Some(value);
        self
    }

    pub fn granularity_seconds(mut self, value: i64) -> Self {
        self.granularity_seconds = Some(value);
        self
    }

    pub fn to_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AggregationMeta`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_timestamp`](AggregationMetaBuilder::from_timestamp)
    /// - [`to_timestamp`](AggregationMetaBuilder::to_timestamp)
    pub fn build(self) -> Result<AggregationMeta, BuildError> {
        Ok(AggregationMeta {
            from_timestamp: self
                .from_timestamp
                .ok_or_else(|| BuildError::missing_field("from_timestamp"))?,
            granularity_seconds: self.granularity_seconds,
            to_timestamp: self
                .to_timestamp
                .ok_or_else(|| BuildError::missing_field("to_timestamp"))?,
        })
    }
}
