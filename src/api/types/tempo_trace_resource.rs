pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceResource {
    /// The attributes of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TempoTraceAttribute>>,
}

impl TempoTraceResource {
    pub fn builder() -> TempoTraceResourceBuilder {
        <TempoTraceResourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceResourceBuilder {
    attributes: Option<Vec<TempoTraceAttribute>>,
}

impl TempoTraceResourceBuilder {
    pub fn attributes(mut self, value: Vec<TempoTraceAttribute>) -> Self {
        self.attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceResource`].
    pub fn build(self) -> Result<TempoTraceResource, BuildError> {
        Ok(TempoTraceResource {
            attributes: self.attributes,
        })
    }
}
