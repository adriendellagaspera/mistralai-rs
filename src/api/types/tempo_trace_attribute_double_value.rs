pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct TempoTraceAttributeDoubleValue {
    /// The floating point value of the attribute
    pub double_value: f64,
}

impl TempoTraceAttributeDoubleValue {
    pub fn builder() -> TempoTraceAttributeDoubleValueBuilder {
        <TempoTraceAttributeDoubleValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeDoubleValueBuilder {
    double_value: Option<f64>,
}

impl TempoTraceAttributeDoubleValueBuilder {
    pub fn double_value(mut self, value: f64) -> Self {
        self.double_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeDoubleValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`double_value`](TempoTraceAttributeDoubleValueBuilder::double_value)
    pub fn build(self) -> Result<TempoTraceAttributeDoubleValue, BuildError> {
        Ok(TempoTraceAttributeDoubleValue {
            double_value: self
                .double_value
                .ok_or_else(|| BuildError::missing_field("double_value"))?,
        })
    }
}
