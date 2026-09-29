pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TempoTraceAttributeBoolValue {
    /// The boolean value of the attribute
    pub bool_value: bool,
}

impl TempoTraceAttributeBoolValue {
    pub fn builder() -> TempoTraceAttributeBoolValueBuilder {
        <TempoTraceAttributeBoolValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeBoolValueBuilder {
    bool_value: Option<bool>,
}

impl TempoTraceAttributeBoolValueBuilder {
    pub fn bool_value(mut self, value: bool) -> Self {
        self.bool_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeBoolValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bool_value`](TempoTraceAttributeBoolValueBuilder::bool_value)
    pub fn build(self) -> Result<TempoTraceAttributeBoolValue, BuildError> {
        Ok(TempoTraceAttributeBoolValue {
            bool_value: self
                .bool_value
                .ok_or_else(|| BuildError::missing_field("bool_value"))?,
        })
    }
}
