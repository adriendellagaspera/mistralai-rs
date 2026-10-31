pub use crate::prelude::*;

/// Immutable provenance used to identify a telemetry span.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TelemetrySpanReference {
    #[serde(default)]
    pub span_id: String,
    #[serde(default)]
    pub trace_id: String,
}

impl TelemetrySpanReference {
    pub fn builder() -> TelemetrySpanReferenceBuilder {
        <TelemetrySpanReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TelemetrySpanReferenceBuilder {
    span_id: Option<String>,
    trace_id: Option<String>,
}

impl TelemetrySpanReferenceBuilder {
    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TelemetrySpanReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`span_id`](TelemetrySpanReferenceBuilder::span_id)
    /// - [`trace_id`](TelemetrySpanReferenceBuilder::trace_id)
    pub fn build(self) -> Result<TelemetrySpanReference, BuildError> {
        Ok(TelemetrySpanReference {
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
