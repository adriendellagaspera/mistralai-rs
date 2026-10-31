pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportDatasetFromSpansRequest {
    /// Versioned rules for mapping span or resource attributes into top-level payload keys.
    #[serde(default)]
    pub mapping_contract: SpanDatasetMappingContract,
    /// Spans to attempt to import; unavailable references are skipped, and trace and span IDs are attached as record properties automatically.
    #[serde(default)]
    pub span_references: Vec<TelemetrySpanReference>,
}

impl ImportDatasetFromSpansRequest {
    pub fn builder() -> ImportDatasetFromSpansRequestBuilder {
        <ImportDatasetFromSpansRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportDatasetFromSpansRequestBuilder {
    mapping_contract: Option<SpanDatasetMappingContract>,
    span_references: Option<Vec<TelemetrySpanReference>>,
}

impl ImportDatasetFromSpansRequestBuilder {
    pub fn mapping_contract(mut self, value: SpanDatasetMappingContract) -> Self {
        self.mapping_contract = Some(value);
        self
    }

    pub fn span_references(mut self, value: Vec<TelemetrySpanReference>) -> Self {
        self.span_references = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportDatasetFromSpansRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mapping_contract`](ImportDatasetFromSpansRequestBuilder::mapping_contract)
    /// - [`span_references`](ImportDatasetFromSpansRequestBuilder::span_references)
    pub fn build(self) -> Result<ImportDatasetFromSpansRequest, BuildError> {
        Ok(ImportDatasetFromSpansRequest {
            mapping_contract: self
                .mapping_contract
                .ok_or_else(|| BuildError::missing_field("mapping_contract"))?,
            span_references: self
                .span_references
                .ok_or_else(|| BuildError::missing_field("span_references"))?,
        })
    }
}
