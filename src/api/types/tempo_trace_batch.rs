pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceBatch {
    /// The resource of the batch
    #[serde(default)]
    pub resource: TempoTraceResource,
    /// The spans of the scope
    #[serde(rename = "scopeSpans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_spans: Option<Vec<TempoTraceScopeSpan>>,
}

impl TempoTraceBatch {
    pub fn builder() -> TempoTraceBatchBuilder {
        <TempoTraceBatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceBatchBuilder {
    resource: Option<TempoTraceResource>,
    scope_spans: Option<Vec<TempoTraceScopeSpan>>,
}

impl TempoTraceBatchBuilder {
    pub fn resource(mut self, value: TempoTraceResource) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn scope_spans(mut self, value: Vec<TempoTraceScopeSpan>) -> Self {
        self.scope_spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceBatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource`](TempoTraceBatchBuilder::resource)
    pub fn build(self) -> Result<TempoTraceBatch, BuildError> {
        Ok(TempoTraceBatch {
            resource: self
                .resource
                .ok_or_else(|| BuildError::missing_field("resource"))?,
            scope_spans: self.scope_spans,
        })
    }
}
