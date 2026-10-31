pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineConfigHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl PipelineConfigHeader {
    pub fn builder() -> PipelineConfigHeaderBuilder {
        <PipelineConfigHeaderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineConfigHeaderBuilder {
    value: Option<String>,
}

impl PipelineConfigHeaderBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineConfigHeader`].
    pub fn build(self) -> Result<PipelineConfigHeader, BuildError> {
        Ok(PipelineConfigHeader { value: self.value })
    }
}
