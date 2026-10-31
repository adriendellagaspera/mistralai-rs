pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PipelineConfigSelector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    pub source_kind: SourceKind,
}

impl PipelineConfigSelector {
    pub fn builder() -> PipelineConfigSelectorBuilder {
        <PipelineConfigSelectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineConfigSelectorBuilder {
    filter: Option<String>,
    source_kind: Option<SourceKind>,
}

impl PipelineConfigSelectorBuilder {
    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = Some(value.into());
        self
    }

    pub fn source_kind(mut self, value: SourceKind) -> Self {
        self.source_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineConfigSelector`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_kind`](PipelineConfigSelectorBuilder::source_kind)
    pub fn build(self) -> Result<PipelineConfigSelector, BuildError> {
        Ok(PipelineConfigSelector {
            filter: self.filter,
            source_kind: self
                .source_kind
                .ok_or_else(|| BuildError::missing_field("source_kind"))?,
        })
    }
}
