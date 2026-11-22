pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaObservabilityEvaluationsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl BetaObservabilityEvaluationsListQueryRequest {
    pub fn builder() -> BetaObservabilityEvaluationsListQueryRequestBuilder {
        <BetaObservabilityEvaluationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaObservabilityEvaluationsListQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
}

impl BetaObservabilityEvaluationsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaObservabilityEvaluationsListQueryRequest`].
    pub fn build(self) -> Result<BetaObservabilityEvaluationsListQueryRequest, BuildError> {
        Ok(BetaObservabilityEvaluationsListQueryRequest {
            page_size: self.page_size,
            page: self.page,
        })
    }
}
