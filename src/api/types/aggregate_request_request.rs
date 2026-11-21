pub use crate::prelude::*;

/// Request for aggregate (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregateRequest2 {
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    pub body: AggregationRequest,
}

impl AggregateRequest2 {
    pub fn builder() -> AggregateRequest2Builder {
        <AggregateRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregateRequest2Builder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    body: Option<AggregationRequest>,
}

impl AggregateRequest2Builder {
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

    /// Consumes the builder and constructs a [`AggregateRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AggregateRequest2Builder::body)
    pub fn build(self) -> Result<AggregateRequest2, BuildError> {
        Ok(AggregateRequest2 {
            from: self.from,
            to: self.to,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
