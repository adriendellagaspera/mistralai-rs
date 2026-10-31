pub use crate::prelude::*;

/// Mapping rules applied independently to every requested telemetry span.
///
/// Version 1 mappings are optional per span: a missing, null, blank-string, or empty-array
/// source omits the target key. JSON object and array strings become structured values;
/// scalar-looking and malformed JSON strings remain strings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpanDatasetMappingContract {
    #[serde(default)]
    pub mappings: Vec<SpanDatasetMapping>,
    #[serde(default)]
    pub version: i64,
}

impl SpanDatasetMappingContract {
    pub fn builder() -> SpanDatasetMappingContractBuilder {
        <SpanDatasetMappingContractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanDatasetMappingContractBuilder {
    mappings: Option<Vec<SpanDatasetMapping>>,
    version: Option<i64>,
}

impl SpanDatasetMappingContractBuilder {
    pub fn mappings(mut self, value: Vec<SpanDatasetMapping>) -> Self {
        self.mappings = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanDatasetMappingContract`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mappings`](SpanDatasetMappingContractBuilder::mappings)
    /// - [`version`](SpanDatasetMappingContractBuilder::version)
    pub fn build(self) -> Result<SpanDatasetMappingContract, BuildError> {
        Ok(SpanDatasetMappingContract {
            mappings: self
                .mappings
                .ok_or_else(|| BuildError::missing_field("mappings"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
