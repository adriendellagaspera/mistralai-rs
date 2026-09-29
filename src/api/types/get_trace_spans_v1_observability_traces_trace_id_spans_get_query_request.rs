pub use crate::prelude::*;

/// Query parameters for get_trace_spans_v1_observability_traces__trace_id__spans_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequest {
    pub fn builder() -> GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequestBuilder {
        <GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequest, BuildError> {
        Ok(
            GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequest {
                from: self.from,
                to: self.to,
                page_size: self.page_size,
                cursor: self.cursor,
            },
        )
    }
}
