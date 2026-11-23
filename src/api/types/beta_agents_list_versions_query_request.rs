pub use crate::prelude::*;

/// Query parameters for list_versions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaAgentsListVersionsQueryRequest {
    /// Page number (0-indexed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of versions per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl BetaAgentsListVersionsQueryRequest {
    pub fn builder() -> BetaAgentsListVersionsQueryRequestBuilder {
        <BetaAgentsListVersionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaAgentsListVersionsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl BetaAgentsListVersionsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaAgentsListVersionsQueryRequest`].
    pub fn build(self) -> Result<BetaAgentsListVersionsQueryRequest, BuildError> {
        Ok(BetaAgentsListVersionsQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
