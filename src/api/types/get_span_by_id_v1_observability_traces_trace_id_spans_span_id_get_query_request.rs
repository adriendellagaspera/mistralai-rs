pub use crate::prelude::*;

/// Query parameters for get_span_by_id_v1_observability_traces__trace_id__spans__span_id__get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequest {
    pub fn builder() -> GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequestBuilder {
        <GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequest, BuildError> {
        Ok(
            GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequest {
                from: self.from,
                to: self.to,
            },
        )
    }
}
