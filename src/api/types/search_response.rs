pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchResponse {
    #[serde(default)]
    pub hits: Vec<SearchHitResponse>,
}

impl SearchResponse {
    pub fn builder() -> SearchResponseBuilder {
        <SearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResponseBuilder {
    hits: Option<Vec<SearchHitResponse>>,
}

impl SearchResponseBuilder {
    pub fn hits(mut self, value: Vec<SearchHitResponse>) -> Self {
        self.hits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hits`](SearchResponseBuilder::hits)
    pub fn build(self) -> Result<SearchResponse, BuildError> {
        Ok(SearchResponse {
            hits: self.hits.ok_or_else(|| BuildError::missing_field("hits"))?,
        })
    }
}
