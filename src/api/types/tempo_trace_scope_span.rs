pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceScopeSpan {
    /// The scope of the span
    #[serde(default)]
    pub scope: TempoTraceScope,
    /// The spans of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<TempoTraceSpan>>,
}

impl TempoTraceScopeSpan {
    pub fn builder() -> TempoTraceScopeSpanBuilder {
        <TempoTraceScopeSpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceScopeSpanBuilder {
    scope: Option<TempoTraceScope>,
    spans: Option<Vec<TempoTraceSpan>>,
}

impl TempoTraceScopeSpanBuilder {
    pub fn scope(mut self, value: TempoTraceScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn spans(mut self, value: Vec<TempoTraceSpan>) -> Self {
        self.spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceScopeSpan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scope`](TempoTraceScopeSpanBuilder::scope)
    pub fn build(self) -> Result<TempoTraceScopeSpan, BuildError> {
        Ok(TempoTraceScopeSpan {
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            spans: self.spans,
        })
    }
}
