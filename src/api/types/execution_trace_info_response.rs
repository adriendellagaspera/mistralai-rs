pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionTraceInfoResponse {
    /// The ID of the trace, if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otel_trace_id: Option<String>,
    /// Whether trace data is available in the trace backend for this execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_trace_data: Option<bool>,
}

impl ExecutionTraceInfoResponse {
    pub fn builder() -> ExecutionTraceInfoResponseBuilder {
        <ExecutionTraceInfoResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionTraceInfoResponseBuilder {
    otel_trace_id: Option<String>,
    has_trace_data: Option<bool>,
}

impl ExecutionTraceInfoResponseBuilder {
    pub fn otel_trace_id(mut self, value: impl Into<String>) -> Self {
        self.otel_trace_id = Some(value.into());
        self
    }

    pub fn has_trace_data(mut self, value: bool) -> Self {
        self.has_trace_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionTraceInfoResponse`].
    pub fn build(self) -> Result<ExecutionTraceInfoResponse, BuildError> {
        Ok(ExecutionTraceInfoResponse {
            otel_trace_id: self.otel_trace_id,
            has_trace_data: self.has_trace_data,
        })
    }
}
