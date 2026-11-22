pub use crate::prelude::*;

/// Request for aggregate_span_evaluations (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregateSpanEvaluationsRequest {
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    pub body: AggregationRequest,
}

impl AggregateSpanEvaluationsRequest {
    pub fn builder() -> AggregateSpanEvaluationsRequestBuilder {
        <AggregateSpanEvaluationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregateSpanEvaluationsRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    body: Option<AggregationRequest>,
}

impl AggregateSpanEvaluationsRequestBuilder {
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

    /// Consumes the builder and constructs a [`AggregateSpanEvaluationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AggregateSpanEvaluationsRequestBuilder::body)
    pub fn build(self) -> Result<AggregateSpanEvaluationsRequest, BuildError> {
        Ok(AggregateSpanEvaluationsRequest {
            from: self.from,
            to: self.to,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
