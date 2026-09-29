pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionTraceInfoResponse {
    /// Whether trace data is available in the trace backend for this execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_trace_data: Option<bool>,
    /// The ID of the trace, if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otel_trace_id: Option<String>,
}

impl ExecutionTraceInfoResponse {
    pub fn builder() -> ExecutionTraceInfoResponseBuilder {
        <ExecutionTraceInfoResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionTraceInfoResponseBuilder {
    has_trace_data: Option<bool>,
    otel_trace_id: Option<String>,
}

impl ExecutionTraceInfoResponseBuilder {
    pub fn has_trace_data(mut self, value: bool) -> Self {
        self.has_trace_data = Some(value);
        self
    }

    pub fn otel_trace_id(mut self, value: impl Into<String>) -> Self {
        self.otel_trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionTraceInfoResponse`].
    pub fn build(self) -> Result<ExecutionTraceInfoResponse, BuildError> {
        Ok(ExecutionTraceInfoResponse {
            has_trace_data: self.has_trace_data,
            otel_trace_id: self.otel_trace_id,
        })
    }
}
