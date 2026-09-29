pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TempoTraceAttribute {
    /// The key of the attribute
    #[serde(default)]
    pub key: String,
    /// The value of the attribute
    pub value: TempoTraceAttributeValue,
}

impl TempoTraceAttribute {
    pub fn builder() -> TempoTraceAttributeBuilder {
        <TempoTraceAttributeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeBuilder {
    key: Option<String>,
    value: Option<TempoTraceAttributeValue>,
}

impl TempoTraceAttributeBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: TempoTraceAttributeValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttribute`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](TempoTraceAttributeBuilder::key)
    /// - [`value`](TempoTraceAttributeBuilder::value)
    pub fn build(self) -> Result<TempoTraceAttribute, BuildError> {
        Ok(TempoTraceAttribute {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
