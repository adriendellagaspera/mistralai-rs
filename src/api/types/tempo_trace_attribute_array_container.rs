pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceAttributeArrayContainer {
    /// The values of the array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<TempoTraceAttributeArrayElement>>,
}

impl TempoTraceAttributeArrayContainer {
    pub fn builder() -> TempoTraceAttributeArrayContainerBuilder {
        <TempoTraceAttributeArrayContainerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeArrayContainerBuilder {
    values: Option<Vec<TempoTraceAttributeArrayElement>>,
}

impl TempoTraceAttributeArrayContainerBuilder {
    pub fn values(mut self, value: Vec<TempoTraceAttributeArrayElement>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeArrayContainer`].
    pub fn build(self) -> Result<TempoTraceAttributeArrayContainer, BuildError> {
        Ok(TempoTraceAttributeArrayContainer {
            values: self.values,
        })
    }
}
