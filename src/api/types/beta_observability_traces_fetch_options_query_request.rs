pub use crate::prelude::*;

/// Query parameters for fetch_options
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaObservabilityTracesFetchOptionsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl BetaObservabilityTracesFetchOptionsQueryRequest {
    pub fn builder() -> BetaObservabilityTracesFetchOptionsQueryRequestBuilder {
        <BetaObservabilityTracesFetchOptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaObservabilityTracesFetchOptionsQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl BetaObservabilityTracesFetchOptionsQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaObservabilityTracesFetchOptionsQueryRequest`].
    pub fn build(self) -> Result<BetaObservabilityTracesFetchOptionsQueryRequest, BuildError> {
        Ok(BetaObservabilityTracesFetchOptionsQueryRequest {
            from: self.from,
            to: self.to,
        })
    }
}
