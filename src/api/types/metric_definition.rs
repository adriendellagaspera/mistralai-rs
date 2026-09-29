pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MetricDefinition {
    #[serde(default)]
    pub measure: String,
    pub aggregation: MetricAggregation,
}

impl MetricDefinition {
    pub fn builder() -> MetricDefinitionBuilder {
        <MetricDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricDefinitionBuilder {
    measure: Option<String>,
    aggregation: Option<MetricAggregation>,
}

impl MetricDefinitionBuilder {
    pub fn measure(mut self, value: impl Into<String>) -> Self {
        self.measure = Some(value.into());
        self
    }

    pub fn aggregation(mut self, value: MetricAggregation) -> Self {
        self.aggregation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`measure`](MetricDefinitionBuilder::measure)
    /// - [`aggregation`](MetricDefinitionBuilder::aggregation)
    pub fn build(self) -> Result<MetricDefinition, BuildError> {
        Ok(MetricDefinition {
            measure: self
                .measure
                .ok_or_else(|| BuildError::missing_field("measure"))?,
            aggregation: self
                .aggregation
                .ok_or_else(|| BuildError::missing_field("aggregation"))?,
        })
    }
}
