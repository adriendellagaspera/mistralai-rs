pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpanDatasetMapping {
    pub source_field: SpanDatasetSourceReference,
    /// Literal top-level key to create in the dataset record payload.
    #[serde(default)]
    pub target_field: String,
}

impl SpanDatasetMapping {
    pub fn builder() -> SpanDatasetMappingBuilder {
        <SpanDatasetMappingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanDatasetMappingBuilder {
    source_field: Option<SpanDatasetSourceReference>,
    target_field: Option<String>,
}

impl SpanDatasetMappingBuilder {
    pub fn source_field(mut self, value: SpanDatasetSourceReference) -> Self {
        self.source_field = Some(value);
        self
    }

    pub fn target_field(mut self, value: impl Into<String>) -> Self {
        self.target_field = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpanDatasetMapping`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_field`](SpanDatasetMappingBuilder::source_field)
    /// - [`target_field`](SpanDatasetMappingBuilder::target_field)
    pub fn build(self) -> Result<SpanDatasetMapping, BuildError> {
        Ok(SpanDatasetMapping {
            source_field: self
                .source_field
                .ok_or_else(|| BuildError::missing_field("source_field"))?,
            target_field: self
                .target_field
                .ok_or_else(|| BuildError::missing_field("target_field"))?,
        })
    }
}
