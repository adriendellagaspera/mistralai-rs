pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TempoTraceAttributeStringValue {
    /// The string value of the attribute
    pub string_value: String,
}

impl TempoTraceAttributeStringValue {
    pub fn builder() -> TempoTraceAttributeStringValueBuilder {
        <TempoTraceAttributeStringValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeStringValueBuilder {
    string_value: Option<String>,
}

impl TempoTraceAttributeStringValueBuilder {
    pub fn string_value(mut self, value: impl Into<String>) -> Self {
        self.string_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeStringValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`string_value`](TempoTraceAttributeStringValueBuilder::string_value)
    pub fn build(self) -> Result<TempoTraceAttributeStringValue, BuildError> {
        Ok(TempoTraceAttributeStringValue {
            string_value: self
                .string_value
                .ok_or_else(|| BuildError::missing_field("string_value"))?,
        })
    }
}
