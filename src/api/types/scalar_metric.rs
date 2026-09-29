pub use crate::prelude::*;

/// Scalar metric with a single value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScalarMetric {
    pub value: ScalarMetricValue,
}

impl ScalarMetric {
    pub fn builder() -> ScalarMetricBuilder {
        <ScalarMetricBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScalarMetricBuilder {
    value: Option<ScalarMetricValue>,
}

impl ScalarMetricBuilder {
    pub fn value(mut self, value: ScalarMetricValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScalarMetric`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ScalarMetricBuilder::value)
    pub fn build(self) -> Result<ScalarMetric, BuildError> {
        Ok(ScalarMetric {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
