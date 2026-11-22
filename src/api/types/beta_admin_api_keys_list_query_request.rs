pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaAdminApiKeysListQueryRequest {
    /// Maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of results to skip before returning results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter API keys by name substring or the last 4 characters of the key. Matching is case-insensitive and accent-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BetaAdminApiKeysListQueryRequest {
    pub fn builder() -> BetaAdminApiKeysListQueryRequestBuilder {
        <BetaAdminApiKeysListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaAdminApiKeysListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    name: Option<String>,
}

impl BetaAdminApiKeysListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BetaAdminApiKeysListQueryRequest`].
    pub fn build(self) -> Result<BetaAdminApiKeysListQueryRequest, BuildError> {
        Ok(BetaAdminApiKeysListQueryRequest {
            limit: self.limit,
            offset: self.offset,
            name: self.name,
        })
    }
}
