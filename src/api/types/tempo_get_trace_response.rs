pub use crate::prelude::*;

/// Trace response in OpenTelemetry format.
///
/// This is the unified trace format used across all trace providers (Tempo, ClickHouse, etc.).
/// Regardless of the underlying backend, all trace data is normalized to this Tempo-compatible
/// OpenTelemetry format to ensure consistency in the API response structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoGetTraceResponse {
    /// The batches of the trace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batches: Option<Vec<TempoTraceBatch>>,
}

impl TempoGetTraceResponse {
    pub fn builder() -> TempoGetTraceResponseBuilder {
        <TempoGetTraceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoGetTraceResponseBuilder {
    batches: Option<Vec<TempoTraceBatch>>,
}

impl TempoGetTraceResponseBuilder {
    pub fn batches(mut self, value: Vec<TempoTraceBatch>) -> Self {
        self.batches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoGetTraceResponse`].
    pub fn build(self) -> Result<TempoGetTraceResponse, BuildError> {
        Ok(TempoGetTraceResponse {
            batches: self.batches,
        })
    }
}
