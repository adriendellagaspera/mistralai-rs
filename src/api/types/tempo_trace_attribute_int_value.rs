pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TempoTraceAttributeIntValue {
    /// The integer value of the attribute
    pub int_value: String,
}

impl TempoTraceAttributeIntValue {
    pub fn builder() -> TempoTraceAttributeIntValueBuilder {
        <TempoTraceAttributeIntValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeIntValueBuilder {
    int_value: Option<String>,
}

impl TempoTraceAttributeIntValueBuilder {
    pub fn int_value(mut self, value: impl Into<String>) -> Self {
        self.int_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeIntValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`int_value`](TempoTraceAttributeIntValueBuilder::int_value)
    pub fn build(self) -> Result<TempoTraceAttributeIntValue, BuildError> {
        Ok(TempoTraceAttributeIntValue {
            int_value: self
                .int_value
                .ok_or_else(|| BuildError::missing_field("int_value"))?,
        })
    }
}
