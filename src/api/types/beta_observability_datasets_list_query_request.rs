pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaObservabilityDatasetsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl BetaObservabilityDatasetsListQueryRequest {
    pub fn builder() -> BetaObservabilityDatasetsListQueryRequestBuilder {
        <BetaObservabilityDatasetsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaObservabilityDatasetsListQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

impl BetaObservabilityDatasetsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BetaObservabilityDatasetsListQueryRequest`].
    pub fn build(self) -> Result<BetaObservabilityDatasetsListQueryRequest, BuildError> {
        Ok(BetaObservabilityDatasetsListQueryRequest {
            page_size: self.page_size,
            page: self.page,
            q: self.q,
        })
    }
}
