pub use crate::prelude::*;

/// Request for aggregate_spans_v1_observability_spans_aggregate_post (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregateSpansV1ObservabilitySpansAggregatePostRequest {
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    pub body: AggregationRequest,
}

impl AggregateSpansV1ObservabilitySpansAggregatePostRequest {
    pub fn builder() -> AggregateSpansV1ObservabilitySpansAggregatePostRequestBuilder {
        <AggregateSpansV1ObservabilitySpansAggregatePostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregateSpansV1ObservabilitySpansAggregatePostRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    body: Option<AggregationRequest>,
}

impl AggregateSpansV1ObservabilitySpansAggregatePostRequestBuilder {
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

    /// Consumes the builder and constructs a [`AggregateSpansV1ObservabilitySpansAggregatePostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AggregateSpansV1ObservabilitySpansAggregatePostRequestBuilder::body)
    pub fn build(
        self,
    ) -> Result<AggregateSpansV1ObservabilitySpansAggregatePostRequest, BuildError> {
        Ok(AggregateSpansV1ObservabilitySpansAggregatePostRequest {
            from: self.from,
            to: self.to,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
