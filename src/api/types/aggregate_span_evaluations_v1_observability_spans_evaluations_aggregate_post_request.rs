pub use crate::prelude::*;

/// Request for aggregate_span_evaluations_v1_observability_spans_evaluations_aggregate_post (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequest {
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    pub body: AggregationRequest,
}

impl AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequest {
    pub fn builder(
    ) -> AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequestBuilder {
        <AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    body: Option<AggregationRequest>,
}

impl AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn body(mut self, value: AggregationRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequestBuilder::body)
    pub fn build(
        self,
    ) -> Result<
        AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequest,
        BuildError,
    > {
        Ok(
            AggregateSpanEvaluationsV1ObservabilitySpansEvaluationsAggregatePostRequest {
                from: self.from,
                to: self.to,
                body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            },
        )
    }
}
