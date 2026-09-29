pub use crate::prelude::*;

/// Request for aggregate_traces_v1_observability_traces_aggregate_post (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregateTracesV1ObservabilityTracesAggregatePostRequest {
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    pub body: AggregationRequest,
}

impl AggregateTracesV1ObservabilityTracesAggregatePostRequest {
    pub fn builder() -> AggregateTracesV1ObservabilityTracesAggregatePostRequestBuilder {
        <AggregateTracesV1ObservabilityTracesAggregatePostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregateTracesV1ObservabilityTracesAggregatePostRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    body: Option<AggregationRequest>,
}

impl AggregateTracesV1ObservabilityTracesAggregatePostRequestBuilder {
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

    /// Consumes the builder and constructs a [`AggregateTracesV1ObservabilityTracesAggregatePostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AggregateTracesV1ObservabilityTracesAggregatePostRequestBuilder::body)
    pub fn build(
        self,
    ) -> Result<AggregateTracesV1ObservabilityTracesAggregatePostRequest, BuildError> {
        Ok(AggregateTracesV1ObservabilityTracesAggregatePostRequest {
            from: self.from,
            to: self.to,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
