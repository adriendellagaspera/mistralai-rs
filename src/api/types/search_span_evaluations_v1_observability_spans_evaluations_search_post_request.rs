pub use crate::prelude::*;

/// Request for search_span_evaluations_v1_observability_spans_evaluations_search_post (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequest {
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub body: SpanEvaluationsRequest,
}

impl SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequest {
    pub fn builder() -> SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequestBuilder
    {
        <SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    page_size: Option<i64>,
    cursor: Option<String>,
    body: Option<SpanEvaluationsRequest>,
}

impl SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequestBuilder {
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

    pub fn body(mut self, value: SpanEvaluationsRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequestBuilder::body)
    pub fn build(
        self,
    ) -> Result<SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequest, BuildError>
    {
        Ok(
            SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequest {
                from: self.from,
                to: self.to,
                page_size: self.page_size,
                cursor: self.cursor,
                body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            },
        )
    }
}
