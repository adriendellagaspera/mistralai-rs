pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TempoTraceAttributeArrayValue {
    /// The array value of the attribute
    pub array_value: TempoTraceAttributeArrayContainer,
}

impl TempoTraceAttributeArrayValue {
    pub fn builder() -> TempoTraceAttributeArrayValueBuilder {
        <TempoTraceAttributeArrayValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeArrayValueBuilder {
    array_value: Option<TempoTraceAttributeArrayContainer>,
}

impl TempoTraceAttributeArrayValueBuilder {
    pub fn array_value(mut self, value: TempoTraceAttributeArrayContainer) -> Self {
        self.array_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeArrayValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`array_value`](TempoTraceAttributeArrayValueBuilder::array_value)
    pub fn build(self) -> Result<TempoTraceAttributeArrayValue, BuildError> {
        Ok(TempoTraceAttributeArrayValue {
            array_value: self
                .array_value
                .ok_or_else(|| BuildError::missing_field("array_value"))?,
        })
    }
}
