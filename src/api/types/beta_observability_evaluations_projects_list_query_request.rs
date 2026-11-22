pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaObservabilityEvaluationsProjectsListQueryRequest {
    /// Case-insensitive filter on project name/slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl BetaObservabilityEvaluationsProjectsListQueryRequest {
    pub fn builder() -> BetaObservabilityEvaluationsProjectsListQueryRequestBuilder {
        <BetaObservabilityEvaluationsProjectsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaObservabilityEvaluationsProjectsListQueryRequestBuilder {
    q: Option<String>,
    page_size: Option<i64>,
    page: Option<i64>,
}

impl BetaObservabilityEvaluationsProjectsListQueryRequestBuilder {
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaObservabilityEvaluationsProjectsListQueryRequest`].
    pub fn build(self) -> Result<BetaObservabilityEvaluationsProjectsListQueryRequest, BuildError> {
        Ok(BetaObservabilityEvaluationsProjectsListQueryRequest {
            q: self.q,
            page_size: self.page_size,
            page: self.page,
        })
    }
}
