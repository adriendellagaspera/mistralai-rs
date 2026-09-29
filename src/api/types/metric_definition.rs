pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MetricDefinition {
    pub aggregation: MetricAggregation,
    #[serde(default)]
    pub measure: String,
}

impl MetricDefinition {
    pub fn builder() -> MetricDefinitionBuilder {
        <MetricDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricDefinitionBuilder {
    aggregation: Option<MetricAggregation>,
    measure: Option<String>,
}

impl MetricDefinitionBuilder {
    pub fn aggregation(mut self, value: MetricAggregation) -> Self {
        self.aggregation = Some(value);
        self
    }

    pub fn measure(mut self, value: impl Into<String>) -> Self {
        self.measure = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MetricDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aggregation`](MetricDefinitionBuilder::aggregation)
    /// - [`measure`](MetricDefinitionBuilder::measure)
    pub fn build(self) -> Result<MetricDefinition, BuildError> {
        Ok(MetricDefinition {
            aggregation: self
                .aggregation
                .ok_or_else(|| BuildError::missing_field("aggregation"))?,
            measure: self
                .measure
                .ok_or_else(|| BuildError::missing_field("measure"))?,
        })
    }
}
