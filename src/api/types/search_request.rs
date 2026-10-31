pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchRequest {
    pub retriever: SearchRequestRetriever,
}

impl SearchRequest {
    pub fn builder() -> SearchRequestBuilder {
        <SearchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchRequestBuilder {
    retriever: Option<SearchRequestRetriever>,
}

impl SearchRequestBuilder {
    pub fn retriever(mut self, value: SearchRequestRetriever) -> Self {
        self.retriever = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`retriever`](SearchRequestBuilder::retriever)
    pub fn build(self) -> Result<SearchRequest, BuildError> {
        Ok(SearchRequest {
            retriever: self
                .retriever
                .ok_or_else(|| BuildError::missing_field("retriever"))?,
        })
    }
}
